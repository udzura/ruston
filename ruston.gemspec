lib = File.expand_path("../lib", __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require "ruston/version"

Gem::Specification.new do |spec|
  spec.name          = "ruston"
  spec.version       = Ruston::VERSION
  spec.authors       = ["Uchio Kondo"]
  spec.email         = ["udzura@udzura.jp"]

  spec.summary       = %q{Sample Rust-made gem to parse simple JSON...}
  spec.description   = %q{Sample Rust-made gem to parse simple JSON...}
  spec.homepage      = "https://github.com/udzura/ruston"
  spec.license       = "MIT"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files         = Dir.chdir(File.expand_path('..', __FILE__)) do
    `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  end
  # spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions    = ["Cargo.toml"]
end
