# Formula for the y0sif/homebrew-tap tap. Update url and sha256 on each release.
class Furcule < Formula
  desc "Assumption-first reasoning graph: negate an assumption, see what falls, branch and diff"
  homepage "https://github.com/y0sif/furcule"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/y0sif/furcule/releases/download/v#{version}/furcule-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_ME"
    end
    on_intel do
      url "https://github.com/y0sif/furcule/releases/download/v#{version}/furcule-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_ME"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/y0sif/furcule/releases/download/v#{version}/furcule-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_ME"
    end
    on_intel do
      url "https://github.com/y0sif/furcule/releases/download/v#{version}/furcule-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_ME"
    end
  end

  def install
    bin.install "furcule"
  end

  test do
    assert_match "furcule", shell_output("#{bin}/furcule --version")
  end
end
