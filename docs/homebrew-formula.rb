# Homebrew Formula for cmd-keeper
# This file should be placed in your homebrew-tap repository
# e.g., https://github.com/taro33333/homebrew-tap/Formula/cmd-keeper.rb

class CmdKeeper < Formula
  desc "A CLI tool to save, manage, and search frequently used commands"
  homepage "https://github.com/taro33333/cmd-keeper"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/taro33333/cmd-keeper/releases/download/v#{version}/cmd-keeper-darwin-arm64"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"

      def install
        bin.install "cmd-keeper-darwin-arm64" => "cmd-keeper"
      end
    end

    on_intel do
      url "https://github.com/taro33333/cmd-keeper/releases/download/v#{version}/cmd-keeper-darwin-amd64"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"

      def install
        bin.install "cmd-keeper-darwin-amd64" => "cmd-keeper"
      end
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/taro33333/cmd-keeper/releases/download/v#{version}/cmd-keeper-linux-amd64"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"

      def install
        bin.install "cmd-keeper-linux-amd64" => "cmd-keeper"
      end
    end
  end

  test do
    system "#{bin}/cmd-keeper", "--version"
  end
end

