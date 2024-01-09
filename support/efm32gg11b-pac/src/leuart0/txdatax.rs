#[doc = "Register `TXDATAX` reader"]
pub type R = crate::R<TXDATAX_SPEC>;
#[doc = "Register `TXDATAX` writer"]
pub type W = crate::W<TXDATAX_SPEC>;
#[doc = "Field `TXDATA` reader - TX Data"]
pub type TXDATA_R = crate::FieldReader<u16>;
#[doc = "Field `TXDATA` writer - TX Data"]
pub type TXDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TXBREAK` reader - Transmit Data as Break"]
pub type TXBREAK_R = crate::BitReader;
#[doc = "Field `TXBREAK` writer - Transmit Data as Break"]
pub type TXBREAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISAT` reader - Disable TX After Transmission"]
pub type TXDISAT_R = crate::BitReader;
#[doc = "Field `TXDISAT` writer - Disable TX After Transmission"]
pub type TXDISAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENAT` reader - Enable RX After Transmission"]
pub type RXENAT_R = crate::BitReader;
#[doc = "Field `RXENAT` writer - Enable RX After Transmission"]
pub type RXENAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak(&self) -> TXBREAK_R {
        TXBREAK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Disable TX After Transmission"]
    #[inline(always)]
    pub fn txdisat(&self) -> TXDISAT_R {
        TXDISAT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat(&self) -> RXENAT_R {
        RXENAT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<TXDATAX_SPEC> {
        TXDATA_W::new(self, 0)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    #[must_use]
    pub fn txbreak(&mut self) -> TXBREAK_W<TXDATAX_SPEC> {
        TXBREAK_W::new(self, 13)
    }
    #[doc = "Bit 14 - Disable TX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdisat(&mut self) -> TXDISAT_W<TXDATAX_SPEC> {
        TXDISAT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn rxenat(&mut self) -> RXENAT_W<TXDATAX_SPEC> {
        RXENAT_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Buffer Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdatax::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdatax::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDATAX_SPEC;
impl crate::RegisterSpec for TXDATAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdatax::R`](R) reader structure"]
impl crate::Readable for TXDATAX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdatax::W`](W) writer structure"]
impl crate::Writable for TXDATAX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATAX to value 0"]
impl crate::Resettable for TXDATAX_SPEC {
    const RESET_VALUE: u32 = 0;
}
