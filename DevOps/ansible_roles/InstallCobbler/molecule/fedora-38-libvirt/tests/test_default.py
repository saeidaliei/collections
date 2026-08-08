def test_httpd_installed(host):
    assert host.package("cobbler").is_installed
    assert host.package("tftp-server").is_installed
