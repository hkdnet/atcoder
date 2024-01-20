def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

H, W, K = getis
b = H.times.map do
  gets.chomp.chars
end

def min_op(line)
  l = 0
  min = nil
  while (l + K - 1) < line.size
    tmp = 0
    K.times do |d|
      c = line[l+d]
      if c == 'x'
        l = l+d+1
        tmp = nil
        break
      end

      tmp += 1 if c == '.'
    end

    l += 1
    next if tmp.nil?

    if min.nil? || tmp < min
      min = tmp
    end
  end

  min
end

hm = b.filter_map do |line|
  min_op(line)
end
if hm.size == 0
  h = nil
else
  h = hm.min
end

vm = W.times.filter_map do |w|
  v = b.map { |line| line[w] }
  min_op(v)
end
if vm.size == 0
  v = nil
else
  v = vm.min
end
if h.nil? && v.nil?
  puts "-1"
else
  if h.nil?
    puts v
  else
    if v.nil?
      puts h
    else
      puts [h, v].min
    end
  end
end
