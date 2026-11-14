# alright so plan

## core binary
- serves as the init process.
- [ ] launches all the other binaries in question and wires them up.
- [ ] eventual handoff to one or multiple init services/drivers
## debug uart driver
- basically stdout/in for all the drivers
- [ ] complete
## vfs driver
- equivalent to linuxes global vfs
- [ ] simply emulates a fs layout without actually being a fs or serving file capabilities
- [ ] every folder in of itself implements the same interface so that chroot is as simple as replacing the root vfs driver with a folder in the root