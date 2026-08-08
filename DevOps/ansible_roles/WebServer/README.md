# AnsibleRoleWebServer

Ansible role for installing a web server, either Nginx or Apache.

## Example
An example playbook is provided in:
- [molecule/default/converge.yml](./molecule/default/converge.yml)

## Tests
To run the tests locally:
- `molecule test -s default`

## Notes
- Nginx installation is not meant to be a replacement for the [official Nginx ansible role](https://github.com/nginxinc/ansible-role-nginx/tree/main).
- See [DevEnvSetup](https://gitlab.com/saeidgroup/DevEnvSetup) repository on how to setup molecule for local testing.
