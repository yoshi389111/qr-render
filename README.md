# QR-RENDER(1)

## NAME

`qr-render` - Tool for rendering QR codes in various styles

## SYNOPSIS

```text
qr-render [OPTIONS] <DATA>
```

## DESCRIPTION

`qr-render` is a command-line tool for rendering QR codes in various styles.

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
  - `svg`
- `-o`, `--output <FILE>`: Specify the output file for the QR code. If not provided, the QR code will be printed to stdout.
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

Some styles use Unicode characters that may not be supported by your operating system, terminal emulator, or fonts.
If characters are displayed incorrectly, try a different style.

If Braille is specified for the style, the QR code may not be read correctly in some cases.
In that instance, please use a different style.

## TODO

- Add sixel style support.
- Add support for QR code Level M, Q, and H.
- Add ascii style support.

## COPYRIGHT

&copy; 2026 SATO Yoshiyuki. MIT Licensed.
