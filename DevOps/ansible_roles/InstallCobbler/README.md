# AnsibleRoleCobbler

Ansible role for setting up a minimal netboot environment using Cobbler.

## Example Playbook

An example playbook is provided in:
- [molecule/default/converge.yml](./molecule/default/converge.yml)

## Tests

To run the default test scenario locally:
- `molecule test -s default`

## Notes
If and when services start to fail because of the excessive number of restarts due to 
a lot of systems being added the `StartLimitInterval=` and `StartLimitBurst=` 
should be changed per systemd default or per service, refer to:
[stackoverflow](https://serverfault.com/questions/845471/service-start-request-repeated-too-quickly-refusing-to-start-limit)

## TODO
- provide a set of tested kickstarts for supported distros.
