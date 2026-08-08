def test_dhcp_server_installed(host):
    assert host.package("dhcp-server").is_installed

def test_tftp_server_installed(host):
    assert host.package("tftp-server").is_installed

def test_nfs_server_installed(host):
    assert host.package("nfs-utils").is_installed
