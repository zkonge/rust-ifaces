#![allow(unused, non_upper_case_globals)]

use winapi::basetsd::{ULONG64, UINT8, UINT32};
use winapi::guiddef::GUID;
use winapi::minwindef::{DWORD, BYTE, ULONG, PULONG};
use winapi::winnt::{PCHAR, PWCHAR, WCHAR, PVOID};
use winapi::ws2def::SOCKET_ADDRESS;

const MAX_ADAPTER_ADDRESS_LENGTH: usize = 8;
const ZONE_INDICES_LENGTH: usize = 16;
const MAX_DHCPV6_DUID_LENGTH: usize = 130;
const MAX_DNS_SUFFIX_STRING_LENGTH: usize = 256;

pub const IP_ADAPTER_IPV4_ENABLED: DWORD = 0x0080;
pub const IP_ADAPTER_IPV6_ENABLED: DWORD = 0x0100;

#[link(name = "Iphlpapi")]
extern "system" {
    pub fn GetAdaptersAddresses(family: ULONG,
                                flags: ULONG,
                                reserved: PVOID,
                                addresses: *mut u8,
                                size: PULONG)
                                -> ULONG;
}

#[repr(C)]
pub struct IpAdapterAddresses {
    pub head: IpAdapterAddressesHead,
    pub all: IpAdaptersAddressesAll,
    pub xp: IpAdaptersAddressesXp,
    pub vista: IpAdaptersAddressesVista,
}

#[repr(C)]
pub struct IpAdapterAddressesHead {
    pub length: ULONG,
    if_index: DWORD,
}

/// All Windows & Later
#[repr(C)]
pub struct IpAdaptersAddressesAll {
    pub next: *const IpAdapterAddresses,
    pub adapter_name: PCHAR,
    pub first_unicast_address: *const IpAdapterUnicastAddress,
    first_anycast_address: *const IpAdapterAnycastAddress,
    first_multicast_address: *const IpAdapterMulticastAddress,
    first_dns_server_address: *const IpAdapterDnsServerAddress,
    dns_suffix: PWCHAR,
    pub description: PWCHAR,
    friendly_name: PWCHAR,
    pub physical_address: [BYTE; MAX_ADAPTER_ADDRESS_LENGTH],
    pub physical_address_length: DWORD,
    pub flags: DWORD,
    mtu: DWORD,
    pub if_type: DWORD,
    oper_status: IfOperStatus,
}

/// Windows XP & Later
#[repr(C)]
pub struct IpAdaptersAddressesXp {
    pub ipv6_if_index: DWORD,
    pub zone_indices: [DWORD; ZONE_INDICES_LENGTH],
    first_prefix: *const IpAdapterPrefix,
}

/// Windows Vista & Later
#[repr(C)]
pub struct IpAdaptersAddressesVista {
    transmit_link_speed: ULONG64,
    receive_link_speed: ULONG64,
    first_wins_server_address: *const IpAdapterWinsServerAddress,
    first_gateway_address: *const IpAdapterGatewayAddress,
    ipv4_metric: ULONG,
    ipv6_metric: ULONG,
    luid: IfLuid,
    dhcpv4_server: SOCKET_ADDRESS,
    compartment_id: UINT32,
    network_guid: GUID,
    connection_type: NetIfConnectionType,
    tunnel_type: TunnelType,
    dhcpv6_server: SOCKET_ADDRESS,
    dhcpv6_client_duid: [BYTE; MAX_DHCPV6_DUID_LENGTH],
    dhcpv6_client_duid_length: ULONG,
    dhcpv6_iaid: ULONG,
    first_dns_suffix: *const IpAdapterDnsSuffix,
}

#[repr(C)]
pub struct IpAdapterUnicastAddress {
    pub length: ULONG,
    flags: DWORD,
    pub next: *const IpAdapterUnicastAddress,
    pub address: SOCKET_ADDRESS,
    prefix_origin: IpPrefixOrigin,
    suffix_origin: IpSuffixOrigin,
    pub dad_state: IpDadState,
    valid_lifetime: ULONG,
    preferred_lifetime: ULONG,
    lease_lifetime: ULONG,
    on_link_prefix_length: UINT8,
}

#[repr(C)]
pub struct IpAdapterAnycastAddress {
    length: ULONG,
    flags: DWORD,
    next: *const IpAdapterAnycastAddress,
    address: SOCKET_ADDRESS,
}

#[repr(C)]
pub struct IpAdapterMulticastAddress {
    length: ULONG,
    flags: DWORD,
    next: *const IpAdapterMulticastAddress,
    address: SOCKET_ADDRESS,
}

#[repr(C)]
pub struct IpAdapterDnsServerAddress {
    length: ULONG,
    reserved: DWORD,
    next: *const IpAdapterDnsServerAddress,
    address: SOCKET_ADDRESS,
}

#[repr(C)]
pub struct IpAdapterPrefix {
    length: ULONG,
    flags: DWORD,
    next: *const IpAdapterPrefix,
    address: SOCKET_ADDRESS,
    prefix_length: ULONG,
}

#[repr(C)]
pub struct IpAdapterWinsServerAddress {
    length: ULONG,
    reserved: DWORD,
    next: *const IpAdapterWinsServerAddress,
    address: SOCKET_ADDRESS,
}

#[repr(C)]
pub struct IpAdapterGatewayAddress {
    length: ULONG,
    reserved: DWORD,
    next: *const IpAdapterGatewayAddress,
    address: SOCKET_ADDRESS,
}

#[repr(C)]
pub struct IpAdapterDnsSuffix {
    next: *const IpAdapterDnsSuffix,
    string: [WCHAR; MAX_DNS_SUFFIX_STRING_LENGTH],
}

bitflags! {
    flags IfLuid: ULONG64 {
        const Reserved = 0x0000000000FFFFFF,
        const NetLuidIndex = 0x0000FFFFFF000000,
        const IfType = 0xFFFF000000000000
    }
}

#[repr(C)]
pub enum IpPrefixOrigin {
    IpPrefixOriginOther = 0,
    IpPrefixOriginManual,
    IpPrefixOriginWellKnown,
    IpPrefixOriginDhcp,
    IpPrefixOriginRouterAdvertisement,
    IpPrefixOriginUnchanged = 16,
}

#[repr(C)]
pub enum IpSuffixOrigin {
    IpSuffixOriginOther = 0,
    IpSuffixOriginManual,
    IpSuffixOriginWellKnown,
    IpSuffixOriginDhcp,
    IpSuffixOriginLinkLayerAddress,
    IpSuffixOriginRandom,
    IpSuffixOriginUnchanged = 16,
}

#[derive(PartialEq, Eq)]
#[repr(C)]
pub enum IpDadState {
    IpDadStateInvalid = 0,
    IpDadStateTentative,
    IpDadStateDuplicate,
    IpDadStateDeprecated,
    IpDadStatePreferred,
}

#[repr(C)]
pub enum IfOperStatus {
    IfOperStatusUp = 1,
    IfOperStatusDown = 2,
    IfOperStatusTesting = 3,
    IfOperStatusUnknown = 4,
    IfOperStatusDormant = 5,
    IfOperStatusNotPresent = 6,
    IfOperStatusLowerLayerDown = 7,
}

#[repr(C)]
pub enum NetIfConnectionType {
    NetIfConnectionDedicated = 1,
    NetIfConnectionPassive = 2,
    NetIfConnectionDemand = 3,
    NetIfConnectionMaximum = 4,
}

#[repr(C)]
pub enum TunnelType {
    TunnelTypeNone = 0,
    TunnelTypeOther = 1,
    TunnelTypeDirect = 2,
    TunnelType6To4 = 11,
    TunnelTypeIsatap = 13,
    TunnelTypeTeredo = 14,
    TunnelTypeIpHttps = 15,
}
