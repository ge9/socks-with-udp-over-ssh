# socks-with-udp-over-ssh

Proxies a SOCKS5 proxy with UDP associate over ssh, making it available in the local machine.

# Usage

- Place a `udpsocks-server` binary on an SSH server and make sure it's callable from local machine by `ssh my_server my_command`.
- Set up SOCKS5 with UDP support on the SSH server, and use port forwaring to expose it to the local machine.
- Run `udpsocks-client` to host a SOCKS5 proxy on the local machine.
  - Example commandline: `udpsocks-client 127.0.0.1:1080 -b 127.0.0.1 -l 2080 -h my_server -c sh -c '-i' -c '-l' -c '-c' -c udpsocks-server`

# How it works

It intercepts UDP port notification from the SOCKS5 proxy and use UDP ports on local machine.
UDP packets are translated and go through stdin/stdout.
