
- [ ] make export of png/jpeg/svg work
- [ ] make export of pdf work
- [ ] make export of eps work
- [ ] make export of webp work

- make new template where the entire div is created from javascript  side and/or use fantucini to insert the java call 

- [ ] make fatucinni client session work more then once 

- [ ] implement download of webdriver(s) using something like webdriver_download 

- [ ] deprecate kaleido and remove it from code ...

- [ ] most probably will have to fix the circular dependency

Other esoteric issues;
 - added `percent-encoding = "2.3.1"` because the SVG, when saved via extracting the data from the src element contains URL-encoded data. I am using that package to decode  the url encodings to text  