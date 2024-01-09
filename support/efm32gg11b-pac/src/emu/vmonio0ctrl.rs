#[doc = "Register `VMONIO0CTRL` reader"]
pub type R = crate::R<VMONIO0CTRL_SPEC>;
#[doc = "Register `VMONIO0CTRL` writer"]
pub type W = crate::W<VMONIO0CTRL_SPEC>;
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RISEWU` reader - Rise Wakeup"]
pub type RISEWU_R = crate::BitReader;
#[doc = "Field `RISEWU` writer - Rise Wakeup"]
pub type RISEWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALLWU` reader - Fall Wakeup"]
pub type FALLWU_R = crate::BitReader;
#[doc = "Field `FALLWU` writer - Fall Wakeup"]
pub type FALLWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETDIS` reader - EM4 IO0 Retention Disable"]
pub type RETDIS_R = crate::BitReader;
#[doc = "Field `RETDIS` writer - EM4 IO0 Retention Disable"]
pub type RETDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRESFINE` reader - Threshold Fine Adjust"]
pub type THRESFINE_R = crate::FieldReader;
#[doc = "Field `THRESFINE` writer - Threshold Fine Adjust"]
pub type THRESFINE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `THRESCOARSE` reader - Threshold Coarse Adjust"]
pub type THRESCOARSE_R = crate::FieldReader;
#[doc = "Field `THRESCOARSE` writer - Threshold Coarse Adjust"]
pub type THRESCOARSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    pub fn risewu(&self) -> RISEWU_R {
        RISEWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    pub fn fallwu(&self) -> FALLWU_R {
        FALLWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EM4 IO0 Retention Disable"]
    #[inline(always)]
    pub fn retdis(&self) -> RETDIS_R {
        RETDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    pub fn thresfine(&self) -> THRESFINE_R {
        THRESFINE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn threscoarse(&self) -> THRESCOARSE_R {
        THRESCOARSE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<VMONIO0CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn risewu(&mut self) -> RISEWU_W<VMONIO0CTRL_SPEC> {
        RISEWU_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fallwu(&mut self) -> FALLWU_W<VMONIO0CTRL_SPEC> {
        FALLWU_W::new(self, 3)
    }
    #[doc = "Bit 4 - EM4 IO0 Retention Disable"]
    #[inline(always)]
    #[must_use]
    pub fn retdis(&mut self) -> RETDIS_W<VMONIO0CTRL_SPEC> {
        RETDIS_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn thresfine(&mut self) -> THRESFINE_W<VMONIO0CTRL_SPEC> {
        THRESFINE_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn threscoarse(&mut self) -> THRESCOARSE_W<VMONIO0CTRL_SPEC> {
        THRESCOARSE_W::new(self, 12)
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
#[doc = "VMON IOVDD0 Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmonio0ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmonio0ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMONIO0CTRL_SPEC;
impl crate::RegisterSpec for VMONIO0CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmonio0ctrl::R`](R) reader structure"]
impl crate::Readable for VMONIO0CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vmonio0ctrl::W`](W) writer structure"]
impl crate::Writable for VMONIO0CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VMONIO0CTRL to value 0"]
impl crate::Resettable for VMONIO0CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
