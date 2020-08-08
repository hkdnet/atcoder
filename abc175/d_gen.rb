n = 10
k = rand(20)
ps = (1..n).to_a
cs = n.times.map { rand(100) }

File.open('foo.txt', 'w') do |f|
  f.puts "#{n} #{k}"
  f.puts ps.join(' ')
  f.puts cs.join(' ')
end

def naive
end

actual = `ruby d.rb < foo.txt`.chomp.to_i


