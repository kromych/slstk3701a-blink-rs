#[doc = "Register `ANACTRL` reader"]
pub type R = crate::R<AnactrlSpec>;
#[doc = "Register `ANACTRL` writer"]
pub type W = crate::W<AnactrlSpec>;
#[doc = "Field `IREFPROG` reader - Reference Current Control."]
pub type IrefprogR = crate::FieldReader;
#[doc = "Field `IREFPROG` writer - Reference Current Control."]
pub type IrefprogW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IDACIREFS` reader - Current DAC and Reference Current Scale"]
pub type IdacirefsR = crate::FieldReader;
#[doc = "Field `IDACIREFS` writer - Current DAC and Reference Current Scale"]
pub type IdacirefsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRSTPROG` reader - Reset Timing"]
pub type TrstprogR = crate::FieldReader;
#[doc = "Field `TRSTPROG` writer - Reset Timing"]
pub type TrstprogW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:6 - Reference Current Control."]
    #[inline(always)]
    pub fn irefprog(&self) -> IrefprogR {
        IrefprogR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Current DAC and Reference Current Scale"]
    #[inline(always)]
    pub fn idacirefs(&self) -> IdacirefsR {
        IdacirefsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Reset Timing"]
    #[inline(always)]
    pub fn trstprog(&self) -> TrstprogR {
        TrstprogR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Reference Current Control."]
    #[inline(always)]
    pub fn irefprog(&mut self) -> IrefprogW<'_, AnactrlSpec> {
        IrefprogW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Current DAC and Reference Current Scale"]
    #[inline(always)]
    pub fn idacirefs(&mut self) -> IdacirefsW<'_, AnactrlSpec> {
        IdacirefsW::new(self, 8)
    }
    #[doc = "Bits 20:22 - Reset Timing"]
    #[inline(always)]
    pub fn trstprog(&mut self) -> TrstprogW<'_, AnactrlSpec> {
        TrstprogW::new(self, 20)
    }
}
#[doc = "Analog Control\n\nYou can [`read`](crate::Reg::read) this register and get [`anactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnactrlSpec;
impl crate::RegisterSpec for AnactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anactrl::R`](R) reader structure"]
impl crate::Readable for AnactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`anactrl::W`](W) writer structure"]
impl crate::Writable for AnactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANACTRL to value 0x70"]
impl crate::Resettable for AnactrlSpec {
    const RESET_VALUE: u32 = 0x70;
}
