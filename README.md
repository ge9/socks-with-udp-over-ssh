# socks-with-udp-over-ssh

Proxies a SOCKS5 proxy with UDP associate over ssh, making it available in the local machine. Currently, only IPv4 is supported.

# How it works

It intercepts UDP port notification from the SOCKS5 proxy and use UDP ports on local machine.
UDP packets are translated and go through stdin/stdout.

# Usage

- Place a `udpsocks-server` binary on an SSH server and make sure it's callable from local machine by `my_command` (e.g. `ssh my_server sh -i -l -c udpsocks-server`).
- Set up SOCKS5 with UDP support on the SSH server, and use port forwaring to expose it to the local machine.
  - Example commandline: `ssh -NL 3080:127.0.0.1:1080 my_server`
- Run `udpsocks-client` to host a SOCKS5 proxy on the local machine.
  - Example commandline: `udpsocks-client 127.0.0.1:3080 -b 127.0.0.1 -l 2080 -c ssh -c my_server -c sh -c '-i' -c '-l' -c '-c' -c udpsocks-server`
  - When SOCKS5 is running in ssh client side, use `socat` to convert stdin/stdout to TCP.
    - ssh client side: `socat tcp-listen:2222,fork system:udpsocks-server` and `ssh my_server -NR 3333:127.0.0.1:2222`
      - You may need to use `socat TCP-LISTEN:2222,fork exec:"udpsocks-server",pipes` in Cygwin
    - ssh server side: `udpsocks-client 127.0.0.1:1080 -b 127.0.0.1 -l 2080 -c socat -c - -c "tcp-connect:127.0.0.1:3333"`
- Note: Since only IPv4 is supported, `127.0.0.1` is recommended. `localhost` may not work.

# fake DNS server

The `udpsocks-server` provides a "fake" DNS server at `192.0.2.53:53`, one of special IPv4 addresses for documentation. Any UDP packets to the port are interpreted as DNS request and resolved by `udpsocks-server`. It uses the default domain resolver (like gethostbyname) but you can specify a DNS server explicitly in the argument of `udpsocks-server`. This feature may be useful independently of the main "over-TCP" feature (example commandline: `udpsocks-client 127.0.0.1:1080 -b 127.0.0.1 -l 2080 -c udpsocks-server -c 8.8.8.8:53`).