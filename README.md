# QR-RENDER(1)

## NAME

`qr-render` - Tool for rendering QR codes in various styles

## SYNOPSIS

```text
qr-render [OPTIONS] <DATA>
```

## DESCRIPTION

`qr-render` is a command-line tool for rendering QR codes in various styles, including braille, half, octant, quadrant, separated quadrant, separated sextant, and sextant.

## OPTIONS

- `-q`, `--quiet-zone <SIZE>`: Specify the size of the quiet zone around the QR code. The default size is 2.
- `-s`, `--style <STYLE>`: Specify the rendering style. Available styles are:
  - `braille`
  - `half`
  - `octant`
  - `quadrant`
  - `separated-quadrant`
  - `separated-sextant`
  - `sextant`
- `-h`, `--help`: Show help message and exit.
- `-v`, `--version`: Show version information and exit.

## EXAMPLES

```console
$ qr-render "Hello, World!"

  █▀▀▀▀▀█ █▄ ▀  █▀▀▀▀▀█  
  █ ███ █ ▄█ █  █ ███ █  
  █ ▀▀▀ █  █▀█▄ █ ▀▀▀ █  
  ▀▀▀▀▀▀▀ █▄▀▄█ ▀▀▀▀▀▀▀  
  ▀ █▀▄█▀█▄█ ▀█▄▀  █▄▀█  
  █▀▄ ▀ ▀ ▄███ ▀ ▀▄▄ █▀  
  ▀ ▀▀ ▀▀ ▄ █▄█▄▀▀▄ ▄ █  
  █▀▀▀▀▀█ ███ ▄▄▀▄▀▄▄▄   
  █ ███ █ ▄▀▀ ▄█ ▀▀███   
  █ ▀▀▀ █ ▀ ▄█▀▄██▄ ▀ ▄  
  ▀▀▀▀▀▀▀ ▀ ▀▀▀▀▀▀  ▀    
                         
```

## NOTES

Depending on the Unicode version supported by the operating system, characters used in certain styles may not display correctly.
Please use styles supported by your font or terminal software.

If Braille is specified for the style, the QR code may not be read correctly in some cases.
In that instance, please use a different style.

## TODO

- Add svg style support.
- Add sixel style support.
- Allow specifying file output.

## COPYRIGHT

&copy; 2026 SATO Yoshiyuki. MIT Licensed.
