#[doc = "Register `ETMTECR1` reader"]
pub type R = crate::R<ETMTECR1_SPEC>;
#[doc = "Register `ETMTECR1` writer"]
pub type W = crate::W<ETMTECR1_SPEC>;
#[doc = "Field `ADRCMP` reader - Address Comparator"]
pub type ADRCMP_R = crate::FieldReader;
#[doc = "Field `ADRCMP` writer - Address Comparator"]
pub type ADRCMP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMMAP` reader - Memmap"]
pub type MEMMAP_R = crate::FieldReader<u16>;
#[doc = "Field `MEMMAP` writer - Memmap"]
pub type MEMMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INCEXCTL` reader - Trace Include/Exclude Flag"]
pub type INCEXCTL_R = crate::BitReader;
#[doc = "Field `INCEXCTL` writer - Trace Include/Exclude Flag"]
pub type INCEXCTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCE` reader - Trace Control Enable"]
pub type TCE_R = crate::BitReader;
#[doc = "Field `TCE` writer - Trace Control Enable"]
pub type TCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Address Comparator"]
    #[inline(always)]
    pub fn adrcmp(&self) -> ADRCMP_R {
        ADRCMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Memmap"]
    #[inline(always)]
    pub fn memmap(&self) -> MEMMAP_R {
        MEMMAP_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Trace Include/Exclude Flag"]
    #[inline(always)]
    pub fn incexctl(&self) -> INCEXCTL_R {
        INCEXCTL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Trace Control Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address Comparator"]
    #[inline(always)]
    #[must_use]
    pub fn adrcmp(&mut self) -> ADRCMP_W<ETMTECR1_SPEC> {
        ADRCMP_W::new(self, 0)
    }
    #[doc = "Bits 8:23 - Memmap"]
    #[inline(always)]
    #[must_use]
    pub fn memmap(&mut self) -> MEMMAP_W<ETMTECR1_SPEC> {
        MEMMAP_W::new(self, 8)
    }
    #[doc = "Bit 24 - Trace Include/Exclude Flag"]
    #[inline(always)]
    #[must_use]
    pub fn incexctl(&mut self) -> INCEXCTL_W<ETMTECR1_SPEC> {
        INCEXCTL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Trace Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<ETMTECR1_SPEC> {
        TCE_W::new(self, 25)
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
#[doc = "ETM Trace control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmtecr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmtecr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMTECR1_SPEC;
impl crate::RegisterSpec for ETMTECR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmtecr1::R`](R) reader structure"]
impl crate::Readable for ETMTECR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmtecr1::W`](W) writer structure"]
impl crate::Writable for ETMTECR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMTECR1 to value 0"]
impl crate::Resettable for ETMTECR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
