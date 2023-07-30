# Generic Offline QR Code Generator For Amazing Talker

This project try to provide a safer workaround to an annoying bug in the Amazing Talker website accessed by tablets, which it is impossible to get into the schedule page of the website to start a lesson.

Instead of hopelessly waiting for Amazing Talker devs to fix this critical bug, or using quite dangrous ways to access the zoom meeting (like copying the intivation link into a cloud drive then open that link in my tablet). As a ~~mech birb~~ software developer, I decided to make an offline QR code generator to access the zoom meeting by scanning it; thus, this project doubles as a learning project for understanding fltk-rs, fluid and qr code generator, so that I have a brief understanding to these libraries for my more complex future projects.

This program is simple, what it does is to generate a QR code by clicking a generate button based on my url, no more special then any generic QR code generators.

How to use:
1. Copy the url by right clicking the "Go to Class" Button.
2. Paste your url on the text field, and click generate.
3. QR code is created, so you can scan the QR code using your phones or tablets.
4. If you have done right, a link should be shown and it will redirect you to the zoom meeting page.