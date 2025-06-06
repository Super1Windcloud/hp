function manifest_path($app, $bucket) {
  (Get-ChildItem (Find-BucketDirectory $bucket) -Filter "$(sanitary_path $app).json" -Recurse).FullName
}

function parse_json($path) {
  if ($null -eq $path -or !(Test-Path $path)) { return $null }
  try {
    Get-Content $path -Raw -Encoding UTF8 | ConvertFrom-Json -ErrorAction Stop
  } catch {
    warn "Error parsing JSON at '$path'."
  }
}

function url_manifest($url) {
  $str = $null
  try {
    $wc = New-Object Net.Webclient
    $wc.Headers.Add('User-Agent', (Get-UserAgent))
    $data = $wc.DownloadData($url)
    $str = (Get-Encoding($wc)).GetString($data)
  } catch [system.management.automation.methodinvocationexception] {
    warn "error: $($_.exception.innerexception.message)"
  } catch {
    throw
  }
  if (!$str) { return $null }
  try {
    $str | ConvertFrom-Json -ErrorAction Stop
  } catch {
    warn "Error parsing JSON at '$url'."
  }
}

function Get-Manifest($app) {
  $bucket, $manifest, $url = $null
  $app = $app.TrimStart('/')
  # check if app is a URL or UNC path
  if ($app -match '^(ht|f)tps?://|\\\\') {
    $url = $app
    $app = appname_from_url $url
    $manifest = url_manifest $url
  } else {
    $app, $bucket, $version = parse_app $app
    if ($bucket) {
      $manifest = manifest $app $bucket
    } else {
      foreach ($tekcub in Get-LocalBucket) {
        $manifest = manifest $app $tekcub
        if ($manifest) {
          $bucket = $tekcub
          break
        }
      }
    }
    if (!$manifest) {
      # couldn't find app in buckets: check if it's a local path
      if (Test-Path $app) {
        $url = Convert-Path $app
        $app = appname_from_url $url
        $manifest = url_manifest $url
      } else {
        if (($app -match '\\/') -or $app.EndsWith('.json')) { $url = $app }
        $app = appname_from_url $app
      }
    }
  }
  return $app, $manifest, $bucket, $url
}

function manifest($app, $bucket, $url) {
  if ($url) { return url_manifest $url }
  parse_json (manifest_path $app $bucket)
}

function save_installed_manifest($app, $bucket, $dir, $url) {
  if ($url) {
    $wc = New-Object Net.Webclient
    $wc.Headers.Add('User-Agent', (Get-UserAgent))
    $data = $wc.DownloadData($url)
    (Get-Encoding($wc)).GetString($data) | Out-UTF8File "$dir\manifest.json"
  } else {
    Copy-Item (manifest_path $app $bucket) "$dir\manifest.json"
  }
}

function installed_manifest($app, $version, $global) {
  parse_json "$(versiondir $app $version $global)\manifest.json"
}

function save_install_info($info, $dir) {
  $nulls = $info.keys | Where-Object { $null -eq $info[$_] }
  $nulls | ForEach-Object { $info.remove($_) } # strip null-valued

  $file_content = $info | ConvertToPrettyJson # in 'json.ps1'
  [System.IO.File]::WriteAllLines("$dir\install.json", $file_content)
}

function install_info($app, $version, $global) {
  $path = "$(versiondir $app $version $global)\install.json"
  if (!(Test-Path $path)) { return $null }
  parse_json $path
}

function arch_specific($prop, $manifest, $architecture) {
  if ($manifest.architecture) {
    $val = $manifest.architecture.$architecture.$prop
    if ($val) { return $val } # else fallback to generic prop
  }

  if ($manifest.$prop) { return $manifest.$prop }
}

function Get-SupportedArchitecture($manifest, $architecture) {
  if ($architecture -eq 'arm64' -and ($manifest | ConvertToPrettyJson) -notmatch '[''"]arm64["'']') {
    # Windows 10 enables existing unmodified x86 apps to run on Arm devices.
    # Windows 11 adds the ability to run unmodified x64 Windows apps on Arm devices!
    # Ref: https://learn.microsoft.com/en-us/windows/arm/overview
    if ($WindowsBuild -ge 22000) {
      # Windows 11
      $architecture = '64bit'
    } else {
      # Windows 10
      $architecture = '32bit'
    }
  }
  if (![String]::IsNullOrEmpty((arch_specific 'url' $manifest $architecture))) {
    return $architecture
  }
}

function generate_user_manifest($app, $bucket, $version) {
  # 'autoupdate.ps1' 'buckets.ps1' 'manifest.ps1'
  $app, $manifest, $bucket, $null = Get-Manifest "$bucket/$app"
  if ("$($manifest.version)" -eq "$version") {
    return manifest_path $app $bucket
  }
  warn "Given version ($version) does not match manifest ($($manifest.version))"
  warn "Attempting to generate manifest for '$app' ($version)"

  ensure (usermanifestsdir) | Out-Null
  $manifest_path = "$(usermanifestsdir)\$app.json"

  if (get_config USE_SQLITE_CACHE) {
    $cached_manifest = (Get-ScoopDBItem -Name $app -Bucket $bucket -Version $version).manifest
    if ($cached_manifest) {
      $cached_manifest | Out-UTF8File $manifest_path
      return $manifest_path
    }
  }

  if (!($manifest.autoupdate)) {
    abort "'$app' does not have autoupdate capability`r`ncouldn't find manifest for '$app@$version'"
  }

  try {
    Invoke-AutoUpdate $app $manifest_path $manifest $version $(@{ })
    return $manifest_path
  } catch {
    Write-Host -ForegroundColor DarkRed "Could not install $app@$version"
  }

  return $null
}

function url($manifest, $arch) { arch_specific 'url' $manifest $arch }
function installer($manifest, $arch) { arch_specific 'installer' $manifest $arch }
function uninstaller($manifest, $arch) { arch_specific 'uninstaller' $manifest $arch }
function hash($manifest, $arch) { arch_specific 'hash' $manifest $arch }
function extract_dir($manifest, $arch) { arch_specific 'extract_dir' $manifest $arch }
function extract_to($manifest, $arch) { arch_specific 'extract_to' $manifest $arch }
