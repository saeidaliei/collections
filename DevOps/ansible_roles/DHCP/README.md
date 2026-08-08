# AnsibleRoleTFTP

Ansible role for installing a DHCP server.

## Example Playbook

An example playbook is provided in:
- [molecule/default/converge.tml](./molecule/default/converge.yml)

## Tests

To run the default test scenario locally:
- `molecule test -s default`

## Notes

- `dhcpd_is_pxe` variable is set to `true`, which means we assume this is for 
pxe boot environment setup, otherwise set to `false`, this will change the `dhcpd.conf` file.
