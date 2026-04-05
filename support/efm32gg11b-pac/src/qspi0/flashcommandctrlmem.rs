#[doc = "Register `FLASHCOMMANDCTRLMEM` reader"]
pub type R = crate::R<FlashcommandctrlmemSpec>;
#[doc = "Register `FLASHCOMMANDCTRLMEM` writer"]
pub type W = crate::W<FlashcommandctrlmemSpec>;
#[doc = "Field `TRIGGERMEMBANKREQ` writer - Trigger the Memory Bank Data Request"]
pub type TriggermembankreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMBANKREQINPROGRESS` reader - Memory Bank Data Request in Progress"]
pub type MembankreqinprogressR = crate::BitReader;
#[doc = "Field `MEMBANKREADDATA` reader - Last Requested Data From the STIG Memory Bank"]
pub type MembankreaddataR = crate::FieldReader;
#[doc = "Field `NBOFSTIGREADBYTES` reader - Number of Read Bytes for the Extended STIG"]
pub type NbofstigreadbytesR = crate::FieldReader;
#[doc = "Field `NBOFSTIGREADBYTES` writer - Number of Read Bytes for the Extended STIG"]
pub type NbofstigreadbytesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEMBANKADDR` reader - Memory Bank Address"]
pub type MembankaddrR = crate::FieldReader<u16>;
#[doc = "Field `MEMBANKADDR` writer - Memory Bank Address"]
pub type MembankaddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 1 - Memory Bank Data Request in Progress"]
    #[inline(always)]
    pub fn membankreqinprogress(&self) -> MembankreqinprogressR {
        MembankreqinprogressR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Last Requested Data From the STIG Memory Bank"]
    #[inline(always)]
    pub fn membankreaddata(&self) -> MembankreaddataR {
        MembankreaddataR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Number of Read Bytes for the Extended STIG"]
    #[inline(always)]
    pub fn nbofstigreadbytes(&self) -> NbofstigreadbytesR {
        NbofstigreadbytesR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:28 - Memory Bank Address"]
    #[inline(always)]
    pub fn membankaddr(&self) -> MembankaddrR {
        MembankaddrR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger the Memory Bank Data Request"]
    #[inline(always)]
    pub fn triggermembankreq(&mut self) -> TriggermembankreqW<'_, FlashcommandctrlmemSpec> {
        TriggermembankreqW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Number of Read Bytes for the Extended STIG"]
    #[inline(always)]
    pub fn nbofstigreadbytes(&mut self) -> NbofstigreadbytesW<'_, FlashcommandctrlmemSpec> {
        NbofstigreadbytesW::new(self, 16)
    }
    #[doc = "Bits 20:28 - Memory Bank Address"]
    #[inline(always)]
    pub fn membankaddr(&mut self) -> MembankaddrW<'_, FlashcommandctrlmemSpec> {
        MembankaddrW::new(self, 20)
    }
}
#[doc = "Flash Command Control Memory Register (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcommandctrlmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcommandctrlmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcommandctrlmemSpec;
impl crate::RegisterSpec for FlashcommandctrlmemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcommandctrlmem::R`](R) reader structure"]
impl crate::Readable for FlashcommandctrlmemSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcommandctrlmem::W`](W) writer structure"]
impl crate::Writable for FlashcommandctrlmemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCOMMANDCTRLMEM to value 0"]
impl crate::Resettable for FlashcommandctrlmemSpec {}
