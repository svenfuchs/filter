package main

import (
	"bytes"
	"fmt"
	"io"
	"reflect"
	"sort"
	"testing"
)

func assert(t *testing.T, a interface{}) {
	assertEqual(t, a, true)
}

func assertFalse(t *testing.T, a interface{}) {
	assertEqual(t, a, false)
}

func assertEqual(t *testing.T, a interface{}, b interface{}) {
	if !reflect.DeepEqual(a, b) {
		msg := fmt.Sprintf("Expected %v to equal %v, but didn't", a, b)
		t.Fatal(msg)
	}
}

func TestChoose(t *testing.T) {
	strs := []string{"a", "bb", "ccc", "dddd"}
	strs = choose(strs, func(str string) bool { return len(str) > 2 })
	assertEqual(t, strs, []string{"ccc", "dddd"})
}

func TestContains(t *testing.T) {
	strs := []string{"a", "b", "c"}
	assert(t, contains(strs, "a"))
	assert(t, contains(strs, "c"))
	assertFalse(t, contains(strs, "d"))
}

func TestUniq(t *testing.T) {
	strs := []string{"a", "b", "a", "c", "b", "b"}
	strs = uniq(strs)
	assertEqual(t, strs, []string{"a", "b", "c"})
}

func TestByLength(t *testing.T) {
	strs := []string{"aaaa", "a", "aa", "aaa"}
	sort.Sort(byLength(strs))
	assertEqual(t, strs, []string{"a", "aa", "aaa", "aaaa"})
}

func TestByReverseLength(t *testing.T) {
	strs := []string{"aaaa", "a", "aa", "aaa"}
	sort.Sort(sort.Reverse(byLength(strs)))
	assertEqual(t, strs, []string{"aaaa", "aaa", "aa", "a"})
}

func TestUnescape(t *testing.T) {
	str := unescape(`foo\ bar`)
	assertEqual(t, "foo bar", str)
}

func TestMapStrs(t *testing.T) {
	strs := []string{"a ", "b "}
	strs = mapStrs(strs, func(str string) string { return string(str[0]) })
	assertEqual(t, strs, []string{"a", "b"})
}

func TestStdinReadChar(t *testing.T) {
	in := stdin{in: bytes.NewBufferString("abc")}
	char, _ := in.read()
	assertEqual(t, "a", string(char))
}

func TestStdinReadAll(t *testing.T) {
	in := stdin{in: bytes.NewBufferString("abc")}
	buf := []byte{}
	for {
		char, err := in.read()
		if err == io.EOF {
			break
		}
		buf = append(buf, append(char, byte('.'))...)
	}
	assertEqual(t, "a.b.c.", string(buf))
}

func TestFilter(t *testing.T) {
	in := readable(stdin{in: bytes.NewBufferString("foo bar baz buz")})
	in = filter{in: in, str: "foo", mask: []byte("[.]")}
	in = filter{in: in, str: "baz", mask: []byte("[.]")}
	buf := ""
	for {
		str, err := in.read()
		if err == io.EOF {
			break
		}
		buf = buf + string(str)
	}
	assertEqual(t, "[.] bar [.] buz", buf)
}

func TestFiltered(t *testing.T) {
	strs := []string{"foo", "baz"}
	in := bytes.NewBufferString("foo bar baz buz")
	out := bytes.NewBufferString("")
	filtered(in, out, strs, "[.]").run()
	assertEqual(t, "[.] bar [.] buz", string(out.Bytes()))
}
