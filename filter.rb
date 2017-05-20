class Stdin < Struct.new(:io)
  def read
    io.readchar unless io.eof?
  end
end

class Filter < Struct.new(:reader, :str, :mask)
  def read
    while char = reader.read
      buffer << char
      if str == buffer
        return mask
      elsif !str.start_with?(buffer)
        return flush
      end
    end
  end

  def mask
    buffer.clear
    super
  end

  def flush
    str = buffer.dup
    buffer.clear
    return str
  end

  def buffer
    @buffer ||= ''
  end
end

def write(reader, writer)
  while char = reader.read
    writer.print(char)
  end
end

def unescape(str)
  `echo #{str}`.chomp rescue ''
end

strs = ARGV.map { |key| ENV[key] }.compact
strs = strs.reject { |s| s.length < 3 }
strs = strs.map { |s| [s, unescape(s)] }.flatten
strs = strs.uniq.sort_by { |s| -s.length }

reader = strs.inject(Stdin.new($stdin)) do |reader, str|
  Filter.new(reader, str, '[.]')
end
write(reader, $stdout)
