#[doc = "Register `DEVSIZECONFIG` reader"]
pub type R = crate::R<DEVSIZECONFIG_SPEC>;
#[doc = "Register `DEVSIZECONFIG` writer"]
pub type W = crate::W<DEVSIZECONFIG_SPEC>;
#[doc = "Field `NUMADDRBYTES` reader - Number of Address Bytes"]
pub type NUMADDRBYTES_R = crate::FieldReader;
#[doc = "Field `NUMADDRBYTES` writer - Number of Address Bytes"]
pub type NUMADDRBYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BYTESPERDEVICEPAGE` reader - Number of Bytes Per Device Page"]
pub type BYTESPERDEVICEPAGE_R = crate::FieldReader<u16>;
#[doc = "Field `BYTESPERDEVICEPAGE` writer - Number of Bytes Per Device Page"]
pub type BYTESPERDEVICEPAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BYTESPERSUBSECTOR` reader - Number of Bytes Per Block"]
pub type BYTESPERSUBSECTOR_R = crate::FieldReader;
#[doc = "Field `BYTESPERSUBSECTOR` writer - Number of Bytes Per Block"]
pub type BYTESPERSUBSECTOR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MEMSIZEONCS0` reader - Size of Flash Device Connected to CS\\[0\\]
Pin"]
pub type MEMSIZEONCS0_R = crate::FieldReader;
#[doc = "Field `MEMSIZEONCS0` writer - Size of Flash Device Connected to CS\\[0\\]
Pin"]
pub type MEMSIZEONCS0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEMSIZEONCS1` reader - Size of Flash Device Connected to CS\\[1\\]
Pin"]
pub type MEMSIZEONCS1_R = crate::FieldReader;
#[doc = "Field `MEMSIZEONCS1` writer - Size of Flash Device Connected to CS\\[1\\]
Pin"]
pub type MEMSIZEONCS1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&self) -> NUMADDRBYTES_R {
        NUMADDRBYTES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Number of Bytes Per Device Page"]
    #[inline(always)]
    pub fn bytesperdevicepage(&self) -> BYTESPERDEVICEPAGE_R {
        BYTESPERDEVICEPAGE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:20 - Number of Bytes Per Block"]
    #[inline(always)]
    pub fn bytespersubsector(&self) -> BYTESPERSUBSECTOR_R {
        BYTESPERSUBSECTOR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Size of Flash Device Connected to CS\\[0\\]
Pin"]
    #[inline(always)]
    pub fn memsizeoncs0(&self) -> MEMSIZEONCS0_R {
        MEMSIZEONCS0_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Size of Flash Device Connected to CS\\[1\\]
Pin"]
    #[inline(always)]
    pub fn memsizeoncs1(&self) -> MEMSIZEONCS1_R {
        MEMSIZEONCS1_R::new(((self.bits >> 23) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of Address Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn numaddrbytes(&mut self) -> NUMADDRBYTES_W<DEVSIZECONFIG_SPEC> {
        NUMADDRBYTES_W::new(self, 0)
    }
    #[doc = "Bits 4:15 - Number of Bytes Per Device Page"]
    #[inline(always)]
    #[must_use]
    pub fn bytesperdevicepage(&mut self) -> BYTESPERDEVICEPAGE_W<DEVSIZECONFIG_SPEC> {
        BYTESPERDEVICEPAGE_W::new(self, 4)
    }
    #[doc = "Bits 16:20 - Number of Bytes Per Block"]
    #[inline(always)]
    #[must_use]
    pub fn bytespersubsector(&mut self) -> BYTESPERSUBSECTOR_W<DEVSIZECONFIG_SPEC> {
        BYTESPERSUBSECTOR_W::new(self, 16)
    }
    #[doc = "Bits 21:22 - Size of Flash Device Connected to CS\\[0\\]
Pin"]
    #[inline(always)]
    #[must_use]
    pub fn memsizeoncs0(&mut self) -> MEMSIZEONCS0_W<DEVSIZECONFIG_SPEC> {
        MEMSIZEONCS0_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - Size of Flash Device Connected to CS\\[1\\]
Pin"]
    #[inline(always)]
    #[must_use]
    pub fn memsizeoncs1(&mut self) -> MEMSIZEONCS1_W<DEVSIZECONFIG_SPEC> {
        MEMSIZEONCS1_W::new(self, 23)
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
#[doc = "Device Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devsizeconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devsizeconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVSIZECONFIG_SPEC;
impl crate::RegisterSpec for DEVSIZECONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devsizeconfig::R`](R) reader structure"]
impl crate::Readable for DEVSIZECONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devsizeconfig::W`](W) writer structure"]
impl crate::Writable for DEVSIZECONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVSIZECONFIG to value 0x0010_1002"]
impl crate::Resettable for DEVSIZECONFIG_SPEC {
    const RESET_VALUE: u32 = 0x0010_1002;
}
