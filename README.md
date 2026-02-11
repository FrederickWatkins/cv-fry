<img width="250" height="194" alt="fry" src="https://github.com/user-attachments/assets/4f4d18f9-39be-4698-a477-4f445130b93f" />
<img width="375" alt="Screenshot From 2026-02-10 17-25-25" src="https://github.com/user-attachments/assets/dfda52f0-72de-4fb7-9311-83f974e8a843" />
<img width="375" alt="Screenshot From 2026-02-10 18-00-43" src="https://github.com/user-attachments/assets/f6884e1b-59f0-408f-be21-e10d3e8e321b" />

<img width="1000" alt="bee movie" src="https://github.com/user-attachments/assets/28ddf982-c860-43d2-9fbf-cd7282e7e649" />

## TODO list
- Move code into separate crates in cargo workspace to allow incremental building of cpp ☑️ Done!
- Pull registers out of modules and into generic pipeline regs
- Add result forwarding
- Add muldiv instructions
- Separate muldiv into own unit with ready signal to allow variable length pipeline, add valid and ready signals to pipeline regs

## Questions
- Q: Is that your processor printing the entire script of the bee movie to a memory mapped vga buffer? A: Yes.
- Q: If this was a project to learn hardware design why is most of the repo unit tests in rust? A: Unfortunately it seems writing tests is most of the job. If I have to write hundreds of unit tests, I'm not doing it in c++.
- Q: Why is did you name your processor fry? A: It's not very good at anything but I love it anyway.
