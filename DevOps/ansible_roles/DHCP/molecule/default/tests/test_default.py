def test_httpd_installed(host):
    assert host.package("dhcp-server").is_installed
