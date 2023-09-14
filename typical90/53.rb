require "open3"
lines = DATA.read.chomp.split("\n").compact
p lines
Open3.popen3("cargo run --bin 053") do |in_, out, err, thr|
  at_exit do
    in_.close
    err.close
  end
  in_.puts lines.size

  Thread.new do
    loop do
      STDERR.puts err.gets
    rescue
    end
  end
  lines.each do |line|
    arr = line.split(",").map(&:to_i)
    in_.puts arr.size
    loop do
      q = out.gets
      STDERR.puts q
      if q.nil?
        STDERR.puts("got nil")
        exit 1
      end
      if q == "-1"
        STDERR.puts "got -1"
        exit 1
      end
      num = q[2..].to_i
      case q[0]
      when "?"
        STDERR.puts arr[num-1]
        in_.puts arr[num-1]
      when "!"
        expected = arr.max
        if num == expected
          STDERR.puts("ok")
          break
        else
          STDERR.puts("WA: wanted #{expected} but got #{num} for [#{arr.join(",")}]")
          exit 1
        end
      else
        raise "unknown query: #{q}"
      end
    end
  end
end

__END__
1,2,3,4,5,1
1,2,3,4,5,6,9,1
