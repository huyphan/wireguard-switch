# wg-switch

A simple utility command to switch between wireguard profiles.

# Usage

```
Usage: wg-switch <COMMAND>

Commands:
  list    List all profiles
  switch  Switch to a specific profile
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

`wg-switch` requires a configuration file and template file to work. Place the configuration file 
at `/etc/wg-switch/config.json`. Example of a configuration for SurfShark:

```
{
  "template_path": "/etc/wg-switch/template.conf",
  "wg_config_file_path": "/etc/wireguard/wg0.conf",
  "interface": "wg0",
  "profiles": {
    "au-syd": {
      "Endpoint": "au-syd.prod.surfshark.com:51820",
      "PublicKey": "..."
    },
    "uk-lon": {
      "Endpoint": "uk-lon.prod.surfshark.com:51820",
      "PublicKey": "..."
    },
    "us-nyc": {
      "Endpoint": "us-nyc.prod.surfshark.com:51820",
      "PublicKey": "..."
    },
    "vn-hcm": {
      "Endpoint": "vn-hcm.prod.surfshark.com:51820",
      "PublicKey": "..."
    },
    "empty": {
      "Endpoint": "",
      "PublicKey": ""
    }
  }
}
```

Example of a template file:

```
[Interface]
Address = 
ListenPort = 51820
PrivateKey = 

[Peer]
PublicKey = {PublicKey}
AllowedIPs = 0.0.0.0/0
Endpoint = {Endpoint}
```

Only two vairables are supported: `PublicKey` and `Endpoint`.