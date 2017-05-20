package main

import (
	"io"
	"log"
	"os"
	"os/exec"
	"sort"
	"strings"
)

func choose(strs []string, f func(string) bool) []string {
	result := make([]string, 0)
	for _, str := range strs {
		if f(str) {
			result = append(result, str)
		}
	}
	return result
}

func contains(strs []string, str string) bool {
	for _, s := range strs {
		if s == str {
			return true
		}
	}
	return false
}

func uniq(strs []string) []string {
	result := make([]string, 0)
	for _, str := range strs {
		if !contains(result, str) {
			result = append(result, str)
		}
	}
	return result
}

type ByLength []string

func (s ByLength) Len() int {
	return len(s)
}
func (s ByLength) Swap(i, j int) {
	s[i], s[j] = s[j], s[i]
}
func (s ByLength) Less(i, j int) bool {
	return len(s[i]) < len(s[j])
}

func unescape(str string) string {
	out, err := exec.Command("sh", "-c", "echo "+str).Output()
	if err != nil {
		log.Fatal(err)
	}
	return strings.TrimRight(string(out), "\r\n")
}

func mapStrs(strs []string, f func(str string) string) []string {
	result := make([]string, 0)
	for _, str := range strs {
		result = append(result, f(str))
	}
	return result
}

type readable interface {
	read() ([]byte, error)
}

type stdin struct {
	in io.Reader
}

func (r stdin) read() ([]byte, error) {
	char := make([]byte, 1)
	_, err := r.in.Read(char)
	return char, err
}

type filter struct {
	in   readable
	str  string
	buf  []byte
	mask []byte
}

func (f filter) read() ([]byte, error) {
	for {
		char, err := f.in.read()
		if err == io.EOF {
			return f.dump(), err
		}
		f.buf = append(f.buf, char...)
		if f.isSecure() {
			f.clear()
			return f.mask, err
		} else if f.canDump() {
			return f.dump(), err
		}
	}
}

func (f filter) isSecure() bool {
	return string(f.buf) == f.str
}

func (f filter) canDump() bool {
	return !strings.HasPrefix(f.str, string(f.buf))
}

func (f filter) dump() []byte {
	chars := f.buf
	f.clear()
	return chars
}

func (f filter) clear() {
	f.buf = []byte{}
}

type writer struct {
	in  readable
	out io.Writer
}

func (w writer) run() {
	for {
		char, err := w.in.read()
		if err == io.EOF {
			break
		} else if err != nil {
			log.Fatalf("Error reading from input: %s", err)
		} else {
			w.out.Write(char)
		}
	}
}

func args() []string {
	args := os.Args[1:]
	args = choose(args, func(str string) bool { return len(str) > 2 })
	args = append(args, mapStrs(args, unescape)...)
	args = uniq(args)
	sort.Sort(sort.Reverse(ByLength(args)))
	return args
}

func filtered(i io.Reader, o io.Writer, args []string, mask string) writer {
	in := readable(stdin{in: i})
	for _, arg := range args {
		in = filter{in: in, str: arg, mask: []byte(mask)}
	}
	return writer{out: o, in: in}
}

func main() {
	filtered(os.Stdin, os.Stdout, args(), "[secure]").run()
}
