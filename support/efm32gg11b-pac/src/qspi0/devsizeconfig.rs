#[doc = "Register `DEVSIZECONFIG` reader"]
pub type R = crate::R<DevsizeconfigSpec>;
#[doc = "Register `DEVSIZECONFIG` writer"]
pub type W = crate::W<DevsizeconfigSpec>;
#[doc = "Field `NUMADDRBYTES` reader - Number of Address Bytes"]
pub type NumaddrbytesR = crate::FieldReader;
#[doc = "Field `NUMADDRBYTES` writer - Number of Address Bytes"]
pub type NumaddrbytesW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BYTESPERDEVICEPAGE` reader - Number of Bytes Per Device Page"]
pub type BytesperdevicepageR = crate::FieldReader<u16>;
#[doc = "Field `BYTESPERDEVICEPAGE` writer - Number of Bytes Per Device Page"]
pub type BytesperdevicepageW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BYTESPERSUBSECTOR` reader - Number of Bytes Per Block"]
pub type BytespersubsectorR = crate::FieldReader;
#[doc = "Field `BYTESPERSUBSECTOR` writer - Number of Bytes Per Block"]
pub type BytespersubsectorW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MEMSIZEONCS0` reader - Size of Flash Device Connected to CS\\[0\\] Pin"]
pub type Memsizeoncs0R = crate::FieldReader;
#[doc = "Field `MEMSIZEONCS0` writer - Size of Flash Device Connected to CS\\[0\\] Pin"]
pub type Memsizeoncs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEMSIZEONCS1` reader - Size of Flash Device Connected to CS\\[1\\] Pin"]
pub type Memsizeoncs1R = crate::FieldReader;
#[doc = "Field `MEMSIZEONCS1` writer - Size of Flash Device Connected to CS\\[1\\] Pin"]
pub type Memsizeoncs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&self) -> NumaddrbytesR {
        NumaddrbytesR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Number of Bytes Per Device Page"]
    #[inline(always)]
    pub fn bytesperdevicepage(&self) -> BytesperdevicepageR {
        BytesperdevicepageR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:20 - Number of Bytes Per Block"]
    #[inline(always)]
    pub fn bytespersubsector(&self) -> BytespersubsectorR {
        BytespersubsectorR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Size of Flash Device Connected to CS\\[0\\] Pin"]
    #[inline(always)]
    pub fn memsizeoncs0(&self) -> Memsizeoncs0R {
        Memsizeoncs0R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Size of Flash Device Connected to CS\\[1\\] Pin"]
    #[inline(always)]
    pub fn memsizeoncs1(&self) -> Memsizeoncs1R {
        Memsizeoncs1R::new(((self.bits >> 23) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&mut self) -> NumaddrbytesW<'_, DevsizeconfigSpec> {
        NumaddrbytesW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Number of Bytes Per Device Page"]
    #[inline(always)]
    pub fn bytesperdevicepage(&mut self) -> BytesperdevicepageW<'_, DevsizeconfigSpec> {
        BytesperdevicepageW::new(self, 4)
    }
    #[doc = "Bits 16:20 - Number of Bytes Per Block"]
    #[inline(always)]
    pub fn bytespersubsector(&mut self) -> BytespersubsectorW<'_, DevsizeconfigSpec> {
        BytespersubsectorW::new(self, 16)
    }
    #[doc = "Bits 21:22 - Size of Flash Device Connected to CS\\[0\\] Pin"]
    #[inline(always)]
    pub fn memsizeoncs0(&mut self) -> Memsizeoncs0W<'_, DevsizeconfigSpec> {
        Memsizeoncs0W::new(self, 21)
    }
    #[doc = "Bits 23:24 - Size of Flash Device Connected to CS\\[1\\] Pin"]
    #[inline(always)]
    pub fn memsizeoncs1(&mut self) -> Memsizeoncs1W<'_, DevsizeconfigSpec> {
        Memsizeoncs1W::new(self, 23)
    }
}
#[doc = "Device Size Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devsizeconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devsizeconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevsizeconfigSpec;
impl crate::RegisterSpec for DevsizeconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devsizeconfig::R`](R) reader structure"]
impl crate::Readable for DevsizeconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`devsizeconfig::W`](W) writer structure"]
impl crate::Writable for DevsizeconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVSIZECONFIG to value 0x0010_1002"]
impl crate::Resettable for DevsizeconfigSpec {
    const RESET_VALUE: u32 = 0x0010_1002;
}
