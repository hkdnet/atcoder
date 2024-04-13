# file = "test/sample-3.out"
file = "my.out"
_, *lines = File.read(file).chomp.split("\n")


ls = lines.map do |line|
  line.split(" ").first.to_i
end

ls << lines[-1].split(" ").last.to_i

dl = ls.last.to_s.size
bl = ls.last.to_s(2).size
ls.each do |l|
  s = "%#{dl}d  -> %#{bl}s" % [l, l.to_s(2)]
  puts s
end
