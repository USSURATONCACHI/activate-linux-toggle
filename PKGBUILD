# Maintainer: Daniil Redchin <redchindaniil@gmail.com> <github.com/USSURATONCACHI>
pkgname=activate-linux-toggle
pkgver=1.0.1
pkgrel=2
pkgdesc="A simple UI app to enable or disable activate-linux on boot"
arch=('any')
url="https://github.com/USSURATONCACHI/activate-linux-toggle"
license=('Apache-2.0')
provides=('activate-linux-toggle' 'activate-linux-enable' 'activate-linux-disable' 'activate-linux-is-enabled')

depends=('bash' 'coreutils' 'systemd' 'gtk4' 'activate-linux')
makedepends=('coreutils' 'rustup' 'git' 'gcc' 'pkg-config')

source=("${url}/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('37e2e4631469e48c6b360d4e8cf337d467c2d2262b2ea7cb3993a242c913859a')

build() {
    novpn_srcdir="${srcdir}/activate-linux-toggle-${pkgver}"

    rustup default stable

    cd "${novpn_srcdir}" && cargo build --release
}

package() {
    novpn_srcdir="${srcdir}/activate-linux-toggle-${pkgver}"
    
    install -Dm755 "${novpn_srcdir}/target/release/activate-linux-toggle"  "${pkgdir}/usr/bin/activate-linux-toggle"

    install -Dm755 "${novpn_srcdir}/activate-linux-enable"     "${pkgdir}/usr/bin/activate-linux-enable"
    install -Dm755 "${novpn_srcdir}/activate-linux-disable"    "${pkgdir}/usr/bin/activate-linux-disable"
    install -Dm755 "${novpn_srcdir}/activate-linux-is-enabled" "${pkgdir}/usr/bin/activate-linux-is-enabled"

    install -Dm644 "${novpn_srcdir}/activate-linux.service" "${pkgdir}/etc/activate-linux-toggle/activate-linux.service"

    install -Dm644 "${novpn_srcdir}/activate-linux-toggle.desktop" "${pkgdir}/usr/share/applications/activate-linux-toggle.desktop"
}
