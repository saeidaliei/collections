# AnsibleRoleTFTP

Ansible role for installing a Tftp server.

## Example Playbook

An example playbook is provided in:
- [molecule/default/converge.tml](./molecule/default/converge.yml)

## Tests

To run the default test scenario locally:
- `molecule test -s default`

## Notes

- For now providing a specific listen address in vars file results in 
tftp-server not binding to the address, this needs fixing.

