use std::net::{IpAddr, Ipv4Addr};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use trust_dns_client::client::{AsyncClient, ClientHandle};
use trust_dns_client::rr::{DNSClass, Name, RecordType, RData, Record};
use trust_dns_client::udp::UdpClientStream;
use trust_dns_proto::op::{Message, MessageType, OpCode};
use std::str::FromStr;

pub async fn handle_dns_query(query_data: &[u8], upstream_dns: Option<SocketAddr>) -> Option<Vec<u8>> {
    let query = Message::from_vec(query_data).ok()?;
    let queried_name = query.queries().first()?.name().to_utf8();
    // Perform the name resolution
    let resolved_ip = match upstream_dns {
        Some(upstream) => {
            resolve_with_upstream(&queried_name, upstream).await
        },
        None => {
            resolve_locally(&queried_name).await
        }
    };

    let mut response = Message::new();
    response
    .set_id(query.id())
    .set_message_type(MessageType::Response)
    .set_op_code(OpCode::Query)
    .set_authoritative(true)
    .add_query(query.queries().first()?.clone());

    if let Some(ip) = resolved_ip {
        let mut answer = Record::new();
        answer.set_name(Name::from_str(&queried_name).ok()?)
            .set_rr_type(RecordType::A)
            .set_dns_class(DNSClass::IN)
            .set_ttl(60)
            .set_data(Some(RData::A(trust_dns_client::rr::rdata::A(ip))));
        response.add_answer(answer);
    } else {
        response.set_response_code(trust_dns_proto::op::ResponseCode::NXDomain);
    }
    response.to_vec().ok()
}

async fn resolve_with_upstream(domain: &str, upstream: SocketAddr) -> Option<Ipv4Addr> {
    let stream = UdpClientStream::<UdpSocket>::new(upstream);
    let (mut client, bg) = AsyncClient::connect(stream).await.ok()?;
    tokio::spawn(bg);

    let response = client.query(Name::from_str(domain).ok()?, DNSClass::IN, RecordType::A).await.ok()?;
    for answer in response.answers() {
        if let RData::A(ip) = answer.data()? {
            return Some(**ip);
        }
    }
    None
}

async fn resolve_locally(domain: &str) -> Option<Ipv4Addr> {
    let addrs = tokio::net::lookup_host((domain, 0)).await.ok()?;
    for addr in addrs {
        if let IpAddr::V4(ip) = addr.ip() {
            return Some(ip);
        }
    }
    None
}
