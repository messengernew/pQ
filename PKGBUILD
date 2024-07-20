# Maintainer: Quadrat <your_email@example.com>

pkgname=pq
pkgver=latest
pkgrel=1
pkgdesc="aur package updater"
arch=('x86_64')
url="https://github.com/messengernew/pQ"
license=('MIT')
depends=('gcc' 'rust')
source=("pq::https://github.com/messengernew/pQ/releases/latest/download/pQ")
sha256sums=('SKIP')

package() {
    cd "$srcdir"
    install -Dm755 "pq" "$pkgdir/usr/bin/pq"
}