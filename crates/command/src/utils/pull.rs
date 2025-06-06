use crate::config::get_config_value_no_print;
use anyhow::bail;
use clap::Parser;
use git2::{ProxyOptions, Repository};
use std::io::{self, Write};
use std::str;

#[derive(Parser)]
pub struct RepoArgs {
    pub(crate) arg_remote: Option<String>,
    pub(crate) arg_branch: Option<String>,
}

pub type ProgressCallback<'a> = &'a dyn Fn(git2::Progress<'_>, bool) -> bool;
fn do_fetch_default_cli<'a>(
    repo: &'a Repository,
    refs: &[&str],
    remote: &'a mut git2::Remote,
) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
    let mut cb = git2::RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!(
                "Resolving deltas {}/{}\r",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            print!(
                "Received {}/{} objects ({}) in {} bytes\r",
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes()
            );
        }
        io::stdout().flush().unwrap();
        true
    });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb);

    fo.download_tags(git2::AutotagOption::All);
    remote.fetch(refs, Some(&mut fo), None)?;

    let stats = remote.stats();
    if stats.local_objects() > 0 {
        println!(
            "\rReceived {}/{} objects in {} bytes (used {} local \
                objects)",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes(),
            stats.local_objects()
        );
    } else {
        println!(
            "\rReceived {}/{} objects in {} bytes",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes()
        );
    }

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    Ok(repo.reference_to_annotated_commit(&fetch_head)?)
}
fn do_fetch<'a>(
    repo: &'a Repository,
    refs: &[&str],
    remote: &'a mut git2::Remote,
    callback: ProgressCallback<'_>,
) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
    let mut cb = git2::RemoteCallbacks::new();
    cb.transfer_progress(|stats| callback(stats, false));

    let mut proxy_option = ProxyOptions::new();
    let config_proxy = get_config_value_no_print("proxy");
    if !config_proxy.is_empty() {
        let proxy_url = if config_proxy.contains("http://") || config_proxy.contains("https://") {
            config_proxy.clone()
        } else {
            "http://".to_string() + &config_proxy
        };
        proxy_option.url(proxy_url.as_str());
    }
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb);
    fo.proxy_options(proxy_option);
    fo.download_tags(git2::AutotagOption::All);
    remote.fetch(refs, Some(&mut fo), None)?;

    let stats = remote.stats();
    callback(stats, true);
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    Ok(repo.reference_to_annotated_commit(&fetch_head)?)
}

fn fast_forward(
    repo: &Repository,
    lb: &mut git2::Reference,
    rc: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };
    let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
    // println!("{}", msg);

    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(
        git2::build::CheckoutBuilder::default()
            // For some reason the force is required to make the working directory actually get updated
            // I suspect we should be adding some logic to handle dirty working directory states
            // but this is just an examples so maybe not.
            .force(),
    ))?;
    Ok(())
}

fn normal_merge(
    repo: &Repository,
    local: &git2::AnnotatedCommit,
    remote: &git2::AnnotatedCommit,
) -> Result<(), anyhow::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        println!("Merge conflicts detected...");
        repo.checkout_index(Some(&mut idx), None)?;
        bail!("Merge conflicts detected");
    }
    let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
    // now create the merge commit
    let msg = format!("Merge: {} into {}", remote.id(), local.id());
    let sig = repo.signature()?;
    let local_commit = repo.find_commit(local.id())?;
    let remote_commit = repo.find_commit(remote.id())?;
    // Do our merge commit and set current branch head to that commit.
    let _merge_commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &result_tree,
        &[&local_commit, &remote_commit],
    )?;
    // Set working tree to match head.
    repo.checkout_head(None)?;
    Ok(())
}

fn do_merge<'a>(
    repo: &'a Repository,
    remote_branch: &str,
    fetch_commit: git2::AnnotatedCommit<'a>,
) -> Result<(), anyhow::Error> {
    let analysis = repo.merge_analysis(&[&fetch_commit])?;

    if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", remote_branch);
        match repo.find_reference(&refname) {
            Ok(mut r) => {
                fast_forward(repo, &mut r, &fetch_commit)?;
            }
            Err(_) => {
                repo.reference(
                    &refname,
                    fetch_commit.id(),
                    true,
                    &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
                )?;
                repo.set_head(&refname)?;
                repo.checkout_head(Some(
                    git2::build::CheckoutBuilder::default()
                        .allow_conflicts(true)
                        .conflict_style_merge(true)
                        .force(),
                ))?;
            }
        };
    } else if analysis.0.is_normal() {
        // do a normal merge
        let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
        normal_merge(&repo, &head_commit, &fetch_commit)?;
    } else {
        // println!("Nothing to do...");
    }
    Ok(())
}

pub fn run(args: RepoArgs, repo_path: String) -> Result<(), anyhow::Error> {
    let remote_name = args.arg_remote.as_ref().map(|s| &s[..]).unwrap_or("origin");
    let remote_branch = args.arg_branch.as_ref().map(|s| &s[..]).unwrap_or("master");
    let repo = Repository::open(repo_path)?;
    let mut remote = repo.find_remote(remote_name)?;
    let fetch_commit = do_fetch_default_cli(&repo, &[remote_branch], &mut remote)?;
    do_merge(&repo, &remote_branch, fetch_commit)
}

///   当使用indicatif 进度条时 , 如果控制台缓存区输出字符串会导致进度条重新渲染, log::debug,info,warn, println! 等
pub fn run_pull<'a>(
    args: RepoArgs,
    repo_path: String,
    callback: ProgressCallback<'_>,
) -> anyhow::Result<()> {
    let remote_name = args.arg_remote.as_ref().map(|s| &s[..]).unwrap_or("origin");
    let remote_branch = args.arg_branch.as_ref().map(|s| &s[..]).unwrap_or("master");
    let repo = Repository::open(repo_path)?;
    let mut remote = repo.find_remote(remote_name)?;
    let fetch_commit = do_fetch(&repo, &[remote_branch], &mut remote, callback)?;
    do_merge(&repo, &remote_branch, fetch_commit)?;
    Ok(())
}
