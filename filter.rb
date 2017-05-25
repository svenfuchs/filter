def stdin(io)
  -> { io.readchar unless io.eof? }
end

def filter(reader, str, mask)
  -> do
    buffer = ''
    while char = reader.call
      buffer << char
      if str == buffer
        return mask
      elsif !str.start_with?(buffer)
        return buffer
      end
    end
  end
end

def write(reader, writer)
  char = nil
  writer.print(char) while char = reader.call
end

def unescape(str)
  `echo #{str}`.chomp rescue ''
end

strs = ARGV.map { |key| ENV[key] }.compact
strs = strs.reject { |s| s.length < 3 }
strs = strs.map { |s| [s, unescape(s)] }.flatten
strs = strs.uniq.sort_by { |s| -s.length }

reader = strs.inject(stdin($stdin)) do |reader, str|
  filter(reader, str, '[secure]')
end

write(reader, $stdout)
