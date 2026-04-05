#[doc = "Register `SPECADDR3TOP` reader"]
pub type R = crate::R<Specaddr3topSpec>;
#[doc = "Register `SPECADDR3TOP` writer"]
pub type W = crate::W<Specaddr3topSpec>;
#[doc = "Field `ADDR` reader - Specific address 3 MSB"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Specific address 3 MSB"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FILTERTYPE` reader - MAC SA or DA selection"]
pub type FiltertypeR = crate::BitReader;
#[doc = "Field `FILTERTYPE` writer - MAC SA or DA selection"]
pub type FiltertypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTERBYTEMASK` reader - Filter byte Mask"]
pub type FilterbytemaskR = crate::FieldReader;
#[doc = "Field `FILTERBYTEMASK` writer - Filter byte Mask"]
pub type FilterbytemaskW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - Specific address 3 MSB"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    pub fn filtertype(&self) -> FiltertypeR {
        FiltertypeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Filter byte Mask"]
    #[inline(always)]
    pub fn filterbytemask(&self) -> FilterbytemaskR {
        FilterbytemaskR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific address 3 MSB"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Specaddr3topSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    pub fn filtertype(&mut self) -> FiltertypeW<'_, Specaddr3topSpec> {
        FiltertypeW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Filter byte Mask"]
    #[inline(always)]
    pub fn filterbytemask(&mut self) -> FilterbytemaskW<'_, Specaddr3topSpec> {
        FilterbytemaskW::new(self, 24)
    }
}
#[doc = "Specific Address 3 Top\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr3top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr3top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Specaddr3topSpec;
impl crate::RegisterSpec for Specaddr3topSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`specaddr3top::R`](R) reader structure"]
impl crate::Readable for Specaddr3topSpec {}
#[doc = "`write(|w| ..)` method takes [`specaddr3top::W`](W) writer structure"]
impl crate::Writable for Specaddr3topSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPECADDR3TOP to value 0"]
impl crate::Resettable for Specaddr3topSpec {}
