def test_ipxe_installed(host):
    all_vars = host.ansible.get_variables()
    assert 'ipxe_directory' in all_vars
    assert host.file(f"{all_vars['ipxe_directory']}/ipxe.efi").exists
    assert host.file(f"{all_vars['ipxe_directory']}/undionly.kpxe").exists
