# Tutorial: Rust from a scripting background

"If you come from Python, JavaScript, or another scripting language, this
meeting will cover the initial topics that you need to understand to become a
systems programmer using Rust! If you've managed memory in C or C++, some of
these concepts will be review. Intrepid novices learning Rust as their first
programming language are welcome to attend, and pick up the basics as you go.

There will be hands-on examples, so bring your laptop! If you don't have
access to a laptop and would like to borrow one for the meeting, let edunham
know."

## Building Slides

```
$ cd slides
$ virtualenv v
$ source v/bin/activate
(v)$ pip install -r requirements.txt
(v)$ make slides 
(v)$ firefox _build/slides/index.html
(v)$ deactivate # leave virtualenv
```
