def test_httpd_installed(host):
    assert host.package("httpd").is_installed
