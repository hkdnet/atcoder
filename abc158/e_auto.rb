def _solve(n, p, s)
  cnt = 0
  (0..n).each do |l|
    ((l+1)..n).each do |r|
      if (s[l...r].to_i % p) == 0
        cnt += 1
      end
    end
  end
  cnt
end
def solve(s)
  l1, l2 = s.split("\n")
  n, p = l1.split(' ').map(&:to_i)
  _solve(n, p, l2).to_s
end

loop do
input = `ruby e_gen.rb`
File.write('e_in.txt', input)
expected = solve(input)
actual = `cargo run --bin e < e_in.txt`.strip

if expected != actual
  warn "failed with:\n#{input}"

  p expected
  p actual
  exit 1
end

end
