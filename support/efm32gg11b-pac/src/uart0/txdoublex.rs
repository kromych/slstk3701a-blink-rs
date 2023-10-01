#[doc = "Register `TXDOUBLEX` reader"]
pub type R = crate::R<TXDOUBLEX_SPEC>;
#[doc = "Register `TXDOUBLEX` writer"]
pub type W = crate::W<TXDOUBLEX_SPEC>;
#[doc = "Field `TXDATA0` reader - TX Data"]
pub type TXDATA0_R = crate::FieldReader<u16>;
#[doc = "Field `TXDATA0` writer - TX Data"]
pub type TXDATA0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `UBRXAT0` reader - Unblock RX After Transmission"]
pub type UBRXAT0_R = crate::BitReader;
#[doc = "Field `UBRXAT0` writer - Unblock RX After Transmission"]
pub type UBRXAT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXTRIAT0` reader - Set TXTRI After Transmission"]
pub type TXTRIAT0_R = crate::BitReader;
#[doc = "Field `TXTRIAT0` writer - Set TXTRI After Transmission"]
pub type TXTRIAT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXBREAK0` reader - Transmit Data as Break"]
pub type TXBREAK0_R = crate::BitReader;
#[doc = "Field `TXBREAK0` writer - Transmit Data as Break"]
pub type TXBREAK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDISAT0` reader - Clear TXEN After Transmission"]
pub type TXDISAT0_R = crate::BitReader;
#[doc = "Field `TXDISAT0` writer - Clear TXEN After Transmission"]
pub type TXDISAT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXENAT0` reader - Enable RX After Transmission"]
pub type RXENAT0_R = crate::BitReader;
#[doc = "Field `RXENAT0` writer - Enable RX After Transmission"]
pub type RXENAT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDATA1` reader - TX Data"]
pub type TXDATA1_R = crate::FieldReader<u16>;
#[doc = "Field `TXDATA1` writer - TX Data"]
pub type TXDATA1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `UBRXAT1` reader - Unblock RX After Transmission"]
pub type UBRXAT1_R = crate::BitReader;
#[doc = "Field `UBRXAT1` writer - Unblock RX After Transmission"]
pub type UBRXAT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXTRIAT1` reader - Set TXTRI After Transmission"]
pub type TXTRIAT1_R = crate::BitReader;
#[doc = "Field `TXTRIAT1` writer - Set TXTRI After Transmission"]
pub type TXTRIAT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXBREAK1` reader - Transmit Data as Break"]
pub type TXBREAK1_R = crate::BitReader;
#[doc = "Field `TXBREAK1` writer - Transmit Data as Break"]
pub type TXBREAK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDISAT1` reader - Clear TXEN After Transmission"]
pub type TXDISAT1_R = crate::BitReader;
#[doc = "Field `TXDISAT1` writer - Clear TXEN After Transmission"]
pub type TXDISAT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXENAT1` reader - Enable RX After Transmission"]
pub type RXENAT1_R = crate::BitReader;
#[doc = "Field `RXENAT1` writer - Enable RX After Transmission"]
pub type RXENAT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&self) -> TXDATA0_R {
        TXDATA0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat0(&self) -> UBRXAT0_R {
        UBRXAT0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat0(&self) -> TXTRIAT0_R {
        TXTRIAT0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak0(&self) -> TXBREAK0_R {
        TXBREAK0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat0(&self) -> TXDISAT0_R {
        TXDISAT0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat0(&self) -> RXENAT0_R {
        RXENAT0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&self) -> TXDATA1_R {
        TXDATA1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat1(&self) -> UBRXAT1_R {
        UBRXAT1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat1(&self) -> TXTRIAT1_R {
        TXTRIAT1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak1(&self) -> TXBREAK1_R {
        TXBREAK1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat1(&self) -> TXDISAT1_R {
        TXDISAT1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat1(&self) -> RXENAT1_R {
        RXENAT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata0(&mut self) -> TXDATA0_W<TXDOUBLEX_SPEC, 0> {
        TXDATA0_W::new(self)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ubrxat0(&mut self) -> UBRXAT0_W<TXDOUBLEX_SPEC, 11> {
        UBRXAT0_W::new(self)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txtriat0(&mut self) -> TXTRIAT0_W<TXDOUBLEX_SPEC, 12> {
        TXTRIAT0_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    #[must_use]
    pub fn txbreak0(&mut self) -> TXBREAK0_W<TXDOUBLEX_SPEC, 13> {
        TXBREAK0_W::new(self)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdisat0(&mut self) -> TXDISAT0_W<TXDOUBLEX_SPEC, 14> {
        TXDISAT0_W::new(self)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn rxenat0(&mut self) -> RXENAT0_W<TXDOUBLEX_SPEC, 15> {
        RXENAT0_W::new(self)
    }
    #[doc = "Bits 16:24 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata1(&mut self) -> TXDATA1_W<TXDOUBLEX_SPEC, 16> {
        TXDATA1_W::new(self)
    }
    #[doc = "Bit 27 - Unblock RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ubrxat1(&mut self) -> UBRXAT1_W<TXDOUBLEX_SPEC, 27> {
        UBRXAT1_W::new(self)
    }
    #[doc = "Bit 28 - Set TXTRI After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txtriat1(&mut self) -> TXTRIAT1_W<TXDOUBLEX_SPEC, 28> {
        TXTRIAT1_W::new(self)
    }
    #[doc = "Bit 29 - Transmit Data as Break"]
    #[inline(always)]
    #[must_use]
    pub fn txbreak1(&mut self) -> TXBREAK1_W<TXDOUBLEX_SPEC, 29> {
        TXBREAK1_W::new(self)
    }
    #[doc = "Bit 30 - Clear TXEN After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdisat1(&mut self) -> TXDISAT1_W<TXDOUBLEX_SPEC, 30> {
        TXDISAT1_W::new(self)
    }
    #[doc = "Bit 31 - Enable RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn rxenat1(&mut self) -> RXENAT1_W<TXDOUBLEX_SPEC, 31> {
        RXENAT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TX Buffer Double Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdoublex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdoublex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDOUBLEX_SPEC;
impl crate::RegisterSpec for TXDOUBLEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdoublex::R`](R) reader structure"]
impl crate::Readable for TXDOUBLEX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdoublex::W`](W) writer structure"]
impl crate::Writable for TXDOUBLEX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDOUBLEX to value 0"]
impl crate::Resettable for TXDOUBLEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
