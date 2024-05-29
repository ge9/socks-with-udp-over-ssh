# socks-with-udp-over-ssh

This software makes UDP-enabled SOCKS5 proxies available from behind TCP connections like SSH.  It works as a pair of client-side and server-side apps. It is especially useful when you want to make some applications connect networks through machines where you don't have root/admin privilege or right to manage network configurations. Currently, only IPv4 is supported.

# Usage

- Place a `udpsocks-server` binary on an SSH server and make sure it's callable from local machine by `my_command` (e.g. `ssh my_server sh -i -l -c udpsocks-server`).
  - You actually don't need SSH but only TCP connections, but SSH may be required in most situations due to the lack of authentication system in this software.
- Set up SOCKS5 with UDP support on the SSH server or another one that can be fully seen from the SSH server, and use port forwarding to expose it to the local machine.
  - Example commandline: `ssh -NL 3080:1.1.1.1:1080 my_server`
- Run `udpsocks-client` to host a SOCKS5 proxy on the local machine.
  - Example commandline: `udpsocks-client 127.0.0.1:3080 -b 127.0.0.1 -l 2080 -c ssh -c my_server -c sh -c '-i' -c '-l' -c '-c' -c udpsocks-server`
  - When SOCKS5 is running in ssh client side, use `socat` to convert stdin/stdout to TCP.
    - ssh client side: `socat tcp-listen:2222,fork system:udpsocks-server` and `ssh my_server -NR 3333:127.0.0.1:2222`
      - You may need to use `socat TCP-LISTEN:2222,fork exec:"udpsocks-server",pipes` in Cygwin
    - ssh server side: `udpsocks-client 127.0.0.1:1080 -b 127.0.0.1 -l 2080 -c socat -c - -c "tcp-connect:127.0.0.1:3333"`
- Note: Since only IPv4 is supported, `127.0.0.1` is recommended. `localhost` may not work.

# fake DNS server

The `udpsocks-server` provides a "fake" DNS server at `192.0.2.53:53`, one of special IPv4 addresses for documentation. That is, any UDP packets to the port are interpreted as DNS request and resolved by `udpsocks-server`. It uses the default domain resolver (as `gethostbyname()` does) but you can specify a DNS server explicitly in the argument of `udpsocks-server`. This feature can be used independently of the main "over-ssh" feature (example commandline: `udpsocks-client 127.0.0.1:1080 -b 127.0.0.1 -l 2080 -c udpsocks-server -c 8.8.8.8:53`).

# How it works

The client-side app basically just relays packets between clients and the SOCKS5 proxy, but when it captures any UDP associate port notifications from the SOCKS5 proxy, it will assign a local UDP port and reply it instead. Also, it asks the server-side app through stdin/stdout to establish connections to the actual UDP port replied by the original SOCKS5 proxy, and sends UDP packets back and forth by encoding them into the byte stream. 

# Socks5 servers with UDP associate

- [Dante](https://www.inet.no/dante/index.html) (you need udp.connectdst=no for full-cone NAT)
- [3proxy](https://3proxy.ru/)
- https://github.com/haxii/socks5

# Transparent proxy clients
These softwares can force selected applications to send all TCP/UDP packets through SOCKS5 proxies.
- [Proxifyre](https://github.com/wiresock/proxifyre) (Windows)
- [redsocks](https://github.com/semigodking/redsocks) (*nix)
- [hev-socks5-tproxy](https://github.com/heiher/hev-socks5-tproxy) (*nix)