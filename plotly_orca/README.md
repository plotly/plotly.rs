# Plotly Orca
Plotly Orca implements the `orca` feature for [Plotly for Rust](https://github.com/igiagkiozis/plotly)
 
The `orca` feature enables `Plot` conversion to the following output formats: png, jpeg, webp, svg, pdf and eps. 

## Installation instructions
To use `plotly_orca` which is used by the `orca` feature for `plotly`, first you need to install the
[Orca command line utility](https://github.com/plotly/orca). 

Download the appropriate binary of Orca for your system from [here](https://github.com/plotly/orca/releases).

### Linux
Copy the orca-<version>-x86_64.AppImage anywhere in your home directory. 
Say you saved this in: /home/<user_name>/apps/orca-<version>-x86_64.AppImage
Then simply create a symbolic link pointing to the AppImage:

```bash 
sudo ln -s /home/<user_name>/apps/orca-<version>-x86_64.AppImage /usr/bin/plotly_orca
```

Note, it's important that the symbolic link is named exactly as shown above. The name of the link is not `orca` as there 
already exists an executable on RHEL 8 and Centos 8 with that name. 

### MacOSX
TODO

### Windows
TODO