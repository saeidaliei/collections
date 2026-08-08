def test_httpd_installed(host):
    assert host.package("nfs-utils").is_installed
