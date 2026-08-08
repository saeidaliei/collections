# AnsibleRoleNFS

Ansible role for installing a NFS server.

## Example Playbook

An example playbook is provided in:
- [molecule/default/converge.tml](./molecule/default/converge.yml)

## Tests

To run the default test scenario locally:
- `molecule test -s default`

## Notes

- the allowed clients to mount the NFS server share directort is specified with `nfsserver_clients`, 
which by default is everyone, for a more secure setup this should be changed.
