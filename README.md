# socks-with-udp-over-ssh

Proxies a SOCKS5 proxy with UDP associate over ssh, making it available in the local machine.

# How it works

It intercepts UDP port notification from the SOCKS5 proxy and use UDP ports on local machine.
UDP packets are translated and go through stdin/stdout.

# Usage

- Place a `udpsocks-server` binary on an SSH server and make sure it's callable from local machine by `my_command` (e.g. `ssh my_server sh -i -l -c udpsocks-server`).
- Set up SOCKS5 with UDP support on the SSH server, and use port forwaring to expose it to the local machine.
- Run `udpsocks-client` to host a SOCKS5 proxy on the local machine.
  - Example commandline: `udpsocks-client 127.0.0.1:1080 -b 127.0.0.1 -l 2080 -c ssh -c my_server -c sh -c '-i' -c '-l' -c '-c' -c udpsocks-server`
  - When SOCKS5 is running in ssh client side, use `socat` to convert stdin/stdout to TCP.
    - ssh client side: `socat tcp-listen:2222,fork system:udpsocks-server` and `ssh my_server -NR 3333:localhost:2222`
      - You may need to use `socat TCP-LISTEN:2222,fork exec:"udpsocks-server",pipes` in Cygwin
    - ssh server side: `udpsocks-client 127.0.0.1:1080 -b 127.0.0.1 -l 2080 -c socat -c - -c "tcp-connect:127.0.0.1:3333"`
- Note: `127.0.0.1` is recommended. `localhost` may not work.
