def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

T = gets.chomp
N = geti

arr = N.times.map do
  _, *ss = gets.chomp.split(" ")
  ss
end

ps = {"" => 0}
arr.each do |ss|
  nx = {}
  ps.each do |s, cost|
    nx[s] = cost
    ss.each do |sc|
      key = s + sc
      new_val = cost + 1
      if nx[key].nil? || new_val < nx[key]
        nx[key] = new_val
      end
    end
  end
  ps = nx
end

ans = ps[T] || -1
puts ans
