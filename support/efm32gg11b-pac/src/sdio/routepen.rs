#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPEN_SPEC>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPEN_SPEC>;
#[doc = "Field `CLKPEN` reader - CLK I/O Enable"]
pub type CLKPEN_R = crate::BitReader;
#[doc = "Field `CLKPEN` writer - CLK I/O Enable"]
pub type CLKPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDPEN` reader - CMD I/O Enable"]
pub type CMDPEN_R = crate::BitReader;
#[doc = "Field `CMDPEN` writer - CMD I/O Enable"]
pub type CMDPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D0PEN` reader - Dat0 I/O Enable"]
pub type D0PEN_R = crate::BitReader;
#[doc = "Field `D0PEN` writer - Dat0 I/O Enable"]
pub type D0PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D1PEN` reader - Dat1 I/O Enable"]
pub type D1PEN_R = crate::BitReader;
#[doc = "Field `D1PEN` writer - Dat1 I/O Enable"]
pub type D1PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D2PEN` reader - Dat2 I/O Enable"]
pub type D2PEN_R = crate::BitReader;
#[doc = "Field `D2PEN` writer - Dat2 I/O Enable"]
pub type D2PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D3PEN` reader - Dat3 I/O Enable"]
pub type D3PEN_R = crate::BitReader;
#[doc = "Field `D3PEN` writer - Dat3 I/O Enable"]
pub type D3PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D4PEN` reader - Dat4 I/O Enable"]
pub type D4PEN_R = crate::BitReader;
#[doc = "Field `D4PEN` writer - Dat4 I/O Enable"]
pub type D4PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D5PEN` reader - Dat5 Enable"]
pub type D5PEN_R = crate::BitReader;
#[doc = "Field `D5PEN` writer - Dat5 Enable"]
pub type D5PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D6PEN` reader - Dat6 Enable"]
pub type D6PEN_R = crate::BitReader;
#[doc = "Field `D6PEN` writer - Dat6 Enable"]
pub type D6PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D7PEN` reader - Data7 I/O Enable"]
pub type D7PEN_R = crate::BitReader;
#[doc = "Field `D7PEN` writer - Data7 I/O Enable"]
pub type D7PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&self) -> CLKPEN_R {
        CLKPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD I/O Enable"]
    #[inline(always)]
    pub fn cmdpen(&self) -> CMDPEN_R {
        CMDPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dat0 I/O Enable"]
    #[inline(always)]
    pub fn d0pen(&self) -> D0PEN_R {
        D0PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dat1 I/O Enable"]
    #[inline(always)]
    pub fn d1pen(&self) -> D1PEN_R {
        D1PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Dat2 I/O Enable"]
    #[inline(always)]
    pub fn d2pen(&self) -> D2PEN_R {
        D2PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dat3 I/O Enable"]
    #[inline(always)]
    pub fn d3pen(&self) -> D3PEN_R {
        D3PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Dat4 I/O Enable"]
    #[inline(always)]
    pub fn d4pen(&self) -> D4PEN_R {
        D4PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dat5 Enable"]
    #[inline(always)]
    pub fn d5pen(&self) -> D5PEN_R {
        D5PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Dat6 Enable"]
    #[inline(always)]
    pub fn d6pen(&self) -> D6PEN_R {
        D6PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data7 I/O Enable"]
    #[inline(always)]
    pub fn d7pen(&self) -> D7PEN_R {
        D7PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLK I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkpen(&mut self) -> CLKPEN_W<ROUTEPEN_SPEC, 0> {
        CLKPEN_W::new(self)
    }
    #[doc = "Bit 1 - CMD I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdpen(&mut self) -> CMDPEN_W<ROUTEPEN_SPEC, 1> {
        CMDPEN_W::new(self)
    }
    #[doc = "Bit 2 - Dat0 I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn d0pen(&mut self) -> D0PEN_W<ROUTEPEN_SPEC, 2> {
        D0PEN_W::new(self)
    }
    #[doc = "Bit 3 - Dat1 I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1pen(&mut self) -> D1PEN_W<ROUTEPEN_SPEC, 3> {
        D1PEN_W::new(self)
    }
    #[doc = "Bit 4 - Dat2 I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2pen(&mut self) -> D2PEN_W<ROUTEPEN_SPEC, 4> {
        D2PEN_W::new(self)
    }
    #[doc = "Bit 5 - Dat3 I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn d3pen(&mut self) -> D3PEN_W<ROUTEPEN_SPEC, 5> {
        D3PEN_W::new(self)
    }
    #[doc = "Bit 6 - Dat4 I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn d4pen(&mut self) -> D4PEN_W<ROUTEPEN_SPEC, 6> {
        D4PEN_W::new(self)
    }
    #[doc = "Bit 7 - Dat5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn d5pen(&mut self) -> D5PEN_W<ROUTEPEN_SPEC, 7> {
        D5PEN_W::new(self)
    }
    #[doc = "Bit 8 - Dat6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn d6pen(&mut self) -> D6PEN_W<ROUTEPEN_SPEC, 8> {
        D6PEN_W::new(self)
    }
    #[doc = "Bit 9 - Data7 I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn d7pen(&mut self) -> D7PEN_W<ROUTEPEN_SPEC, 9> {
        D7PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I/O LOCATION Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
