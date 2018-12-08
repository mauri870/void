pkgname=void
pkgver=0.1.0
pkgrel=0
arch=('x86_64')
pkgdesc='A simple command line tool that sends data into the void.'
url='https://github.com/mauri870/void'
license=('MIT')
makedepends=('rust' 'git')
source=("${pkgname}::git+${url}.git")
md5sums=("SKIP")

build() {
    cd "${srcdir}/${pkgname}"
    cargo build --release
}

check() {
    cd "${srcdir}/${pkgname}"
    cargo test --release
}

package() {
    cd "${srcdir}/${pkgname}"
    install -Dm755 target/release/"$pkgname" "$pkgdir"/usr/bin/"$pkgname"
}