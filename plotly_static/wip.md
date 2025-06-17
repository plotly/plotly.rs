
- [x] make export of png/jpeg/svg work
- [ ] make export of pdf work
- [ ] make export of eps work
- [ ] make export of webp work
- [ ] add proper documentation to entire new code
- [ ] add a binary as an example to show how to use the new staticly crate even without any plotly specific code, just json stuff 
- [ ] write a nice parser using json serialize/deserialize for capabilities, like you found on the othe rrepo
- [x] make new template where the entire div is created from javascript  side and/or use fantucini to insert the java call 
- [ ] check if logging can be enabled via env variable for the case when the lib is used as a lib and not as a binary   
- [ ] make fatucinni client session work more then once 
- [ ] implement download of webdriver(s) using something like webdriver_download 
- [ ] deprecate kaleido and remove it from code ...
- [ ] most probably will have to fix the circular dependency


Other esoteric issues;
 - added `percent-encoding = "2.3.1"` because the SVG, when saved via extracting the data from the src element contains URL-encoded data. I am using that package to decode  the url encodings to text  

 - replaced percent-encoding with urlenconding