---
name: Bug report
about: Create a report to help us improve
title: "[BUG] "
labels: ''
assignees: ''

---

**To Reproduce**
```rs
use substd::offensivething
offensivething(1,2)
```
panics

**Expected behavior**
offensivething(1,2) is valid and should not panic. 

**Please include the following:**
 - Arch, Triple, or at least OS
 - Version

**Describe the bug**
A clear and concise description of what the bug is. Remember that we intend to panic on invalid input as a feature, and not panic if it's a valid request of the library. If input is valid, it should work.
