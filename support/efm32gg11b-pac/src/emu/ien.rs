#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `VMONAVDDFALL` reader - VMONAVDDFALL Interrupt Enable"]
pub type VmonavddfallR = crate::BitReader;
#[doc = "Field `VMONAVDDFALL` writer - VMONAVDDFALL Interrupt Enable"]
pub type VmonavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONAVDDRISE` reader - VMONAVDDRISE Interrupt Enable"]
pub type VmonavddriseR = crate::BitReader;
#[doc = "Field `VMONAVDDRISE` writer - VMONAVDDRISE Interrupt Enable"]
pub type VmonavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDFALL` reader - VMONALTAVDDFALL Interrupt Enable"]
pub type VmonaltavddfallR = crate::BitReader;
#[doc = "Field `VMONALTAVDDFALL` writer - VMONALTAVDDFALL Interrupt Enable"]
pub type VmonaltavddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDRISE` reader - VMONALTAVDDRISE Interrupt Enable"]
pub type VmonaltavddriseR = crate::BitReader;
#[doc = "Field `VMONALTAVDDRISE` writer - VMONALTAVDDRISE Interrupt Enable"]
pub type VmonaltavddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDFALL` reader - VMONDVDDFALL Interrupt Enable"]
pub type VmondvddfallR = crate::BitReader;
#[doc = "Field `VMONDVDDFALL` writer - VMONDVDDFALL Interrupt Enable"]
pub type VmondvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDRISE` reader - VMONDVDDRISE Interrupt Enable"]
pub type VmondvddriseR = crate::BitReader;
#[doc = "Field `VMONDVDDRISE` writer - VMONDVDDRISE Interrupt Enable"]
pub type VmondvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0FALL` reader - VMONIO0FALL Interrupt Enable"]
pub type Vmonio0fallR = crate::BitReader;
#[doc = "Field `VMONIO0FALL` writer - VMONIO0FALL Interrupt Enable"]
pub type Vmonio0fallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0RISE` reader - VMONIO0RISE Interrupt Enable"]
pub type Vmonio0riseR = crate::BitReader;
#[doc = "Field `VMONIO0RISE` writer - VMONIO0RISE Interrupt Enable"]
pub type Vmonio0riseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO1FALL` reader - VMONIO1FALL Interrupt Enable"]
pub type Vmonio1fallR = crate::BitReader;
#[doc = "Field `VMONIO1FALL` writer - VMONIO1FALL Interrupt Enable"]
pub type Vmonio1fallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO1RISE` reader - VMONIO1RISE Interrupt Enable"]
pub type Vmonio1riseR = crate::BitReader;
#[doc = "Field `VMONIO1RISE` writer - VMONIO1RISE Interrupt Enable"]
pub type Vmonio1riseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5VREADY` reader - R5VREADY Interrupt Enable"]
pub type R5vreadyR = crate::BitReader;
#[doc = "Field `R5VREADY` writer - R5VREADY Interrupt Enable"]
pub type R5vreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONBUVDDFALL` reader - VMONBUVDDFALL Interrupt Enable"]
pub type VmonbuvddfallR = crate::BitReader;
#[doc = "Field `VMONBUVDDFALL` writer - VMONBUVDDFALL Interrupt Enable"]
pub type VmonbuvddfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONBUVDDRISE` reader - VMONBUVDDRISE Interrupt Enable"]
pub type VmonbuvddriseR = crate::BitReader;
#[doc = "Field `VMONBUVDDRISE` writer - VMONBUVDDRISE Interrupt Enable"]
pub type VmonbuvddriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETOVERCURRENTLIMIT` reader - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PfetovercurrentlimitR = crate::BitReader;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFETOVERCURRENTLIMIT` reader - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NfetovercurrentlimitR = crate::BitReader;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NfetovercurrentlimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLPRUNNING` reader - DCDCLPRUNNING Interrupt Enable"]
pub type DcdclprunningR = crate::BitReader;
#[doc = "Field `DCDCLPRUNNING` writer - DCDCLPRUNNING Interrupt Enable"]
pub type DcdclprunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLNRUNNING` reader - DCDCLNRUNNING Interrupt Enable"]
pub type DcdclnrunningR = crate::BitReader;
#[doc = "Field `DCDCLNRUNNING` writer - DCDCLNRUNNING Interrupt Enable"]
pub type DcdclnrunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCINBYPASS` reader - DCDCINBYPASS Interrupt Enable"]
pub type DcdcinbypassR = crate::BitReader;
#[doc = "Field `DCDCINBYPASS` writer - DCDCINBYPASS Interrupt Enable"]
pub type DcdcinbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURDY` reader - BURDY Interrupt Enable"]
pub type BurdyR = crate::BitReader;
#[doc = "Field `BURDY` writer - BURDY Interrupt Enable"]
pub type BurdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5VVSINT` reader - R5VVSINT Interrupt Enable"]
pub type R5vvsintR = crate::BitReader;
#[doc = "Field `R5VVSINT` writer - R5VVSINT Interrupt Enable"]
pub type R5vvsintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` reader - EM23WAKEUP Interrupt Enable"]
pub type Em23wakeupR = crate::BitReader;
#[doc = "Field `EM23WAKEUP` writer - EM23WAKEUP Interrupt Enable"]
pub type Em23wakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSCALEDONE` reader - VSCALEDONE Interrupt Enable"]
pub type VscaledoneR = crate::BitReader;
#[doc = "Field `VSCALEDONE` writer - VSCALEDONE Interrupt Enable"]
pub type VscaledoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` reader - TEMP Interrupt Enable"]
pub type TempR = crate::BitReader;
#[doc = "Field `TEMP` writer - TEMP Interrupt Enable"]
pub type TempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` reader - TEMPLOW Interrupt Enable"]
pub type TemplowR = crate::BitReader;
#[doc = "Field `TEMPLOW` writer - TEMPLOW Interrupt Enable"]
pub type TemplowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` reader - TEMPHIGH Interrupt Enable"]
pub type TemphighR = crate::BitReader;
#[doc = "Field `TEMPHIGH` writer - TEMPHIGH Interrupt Enable"]
pub type TemphighW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddfall(&self) -> VmonavddfallR {
        VmonavddfallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddrise(&self) -> VmonavddriseR {
        VmonavddriseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddfall(&self) -> VmonaltavddfallR {
        VmonaltavddfallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddrise(&self) -> VmonaltavddriseR {
        VmonaltavddriseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddfall(&self) -> VmondvddfallR {
        VmondvddfallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddrise(&self) -> VmondvddriseR {
        VmondvddriseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0fall(&self) -> Vmonio0fallR {
        Vmonio0fallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0rise(&self) -> Vmonio0riseR {
        Vmonio0riseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VMONIO1FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio1fall(&self) -> Vmonio1fallR {
        Vmonio1fallR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VMONIO1RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio1rise(&self) -> Vmonio1riseR {
        Vmonio1riseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - R5VREADY Interrupt Enable"]
    #[inline(always)]
    pub fn r5vready(&self) -> R5vreadyR {
        R5vreadyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - VMONBUVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddfall(&self) -> VmonbuvddfallR {
        VmonbuvddfallR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VMONBUVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddrise(&self) -> VmonbuvddriseR {
        VmonbuvddriseR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&self) -> PfetovercurrentlimitR {
        PfetovercurrentlimitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&self) -> NfetovercurrentlimitR {
        NfetovercurrentlimitR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclprunning(&self) -> DcdclprunningR {
        DcdclprunningR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclnrunning(&self) -> DcdclnrunningR {
        DcdclnrunningR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    pub fn dcdcinbypass(&self) -> DcdcinbypassR {
        DcdcinbypassR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - BURDY Interrupt Enable"]
    #[inline(always)]
    pub fn burdy(&self) -> BurdyR {
        BurdyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - R5VVSINT Interrupt Enable"]
    #[inline(always)]
    pub fn r5vvsint(&self) -> R5vvsintR {
        R5vvsintR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> Em23wakeupR {
        Em23wakeupR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VSCALEDONE Interrupt Enable"]
    #[inline(always)]
    pub fn vscaledone(&self) -> VscaledoneR {
        VscaledoneR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    pub fn templow(&self) -> TemplowR {
        TemplowR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn temphigh(&self) -> TemphighR {
        TemphighR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddfall(&mut self) -> VmonavddfallW<'_, IenSpec> {
        VmonavddfallW::new(self, 0)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddrise(&mut self) -> VmonavddriseW<'_, IenSpec> {
        VmonavddriseW::new(self, 1)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddfall(&mut self) -> VmonaltavddfallW<'_, IenSpec> {
        VmonaltavddfallW::new(self, 2)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddrise(&mut self) -> VmonaltavddriseW<'_, IenSpec> {
        VmonaltavddriseW::new(self, 3)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddfall(&mut self) -> VmondvddfallW<'_, IenSpec> {
        VmondvddfallW::new(self, 4)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddrise(&mut self) -> VmondvddriseW<'_, IenSpec> {
        VmondvddriseW::new(self, 5)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0fall(&mut self) -> Vmonio0fallW<'_, IenSpec> {
        Vmonio0fallW::new(self, 6)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0rise(&mut self) -> Vmonio0riseW<'_, IenSpec> {
        Vmonio0riseW::new(self, 7)
    }
    #[doc = "Bit 8 - VMONIO1FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio1fall(&mut self) -> Vmonio1fallW<'_, IenSpec> {
        Vmonio1fallW::new(self, 8)
    }
    #[doc = "Bit 9 - VMONIO1RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio1rise(&mut self) -> Vmonio1riseW<'_, IenSpec> {
        Vmonio1riseW::new(self, 9)
    }
    #[doc = "Bit 10 - R5VREADY Interrupt Enable"]
    #[inline(always)]
    pub fn r5vready(&mut self) -> R5vreadyW<'_, IenSpec> {
        R5vreadyW::new(self, 10)
    }
    #[doc = "Bit 12 - VMONBUVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddfall(&mut self) -> VmonbuvddfallW<'_, IenSpec> {
        VmonbuvddfallW::new(self, 12)
    }
    #[doc = "Bit 13 - VMONBUVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddrise(&mut self) -> VmonbuvddriseW<'_, IenSpec> {
        VmonbuvddriseW::new(self, 13)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&mut self) -> PfetovercurrentlimitW<'_, IenSpec> {
        PfetovercurrentlimitW::new(self, 16)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&mut self) -> NfetovercurrentlimitW<'_, IenSpec> {
        NfetovercurrentlimitW::new(self, 17)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclprunning(&mut self) -> DcdclprunningW<'_, IenSpec> {
        DcdclprunningW::new(self, 18)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclnrunning(&mut self) -> DcdclnrunningW<'_, IenSpec> {
        DcdclnrunningW::new(self, 19)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    pub fn dcdcinbypass(&mut self) -> DcdcinbypassW<'_, IenSpec> {
        DcdcinbypassW::new(self, 20)
    }
    #[doc = "Bit 22 - BURDY Interrupt Enable"]
    #[inline(always)]
    pub fn burdy(&mut self) -> BurdyW<'_, IenSpec> {
        BurdyW::new(self, 22)
    }
    #[doc = "Bit 23 - R5VVSINT Interrupt Enable"]
    #[inline(always)]
    pub fn r5vvsint(&mut self) -> R5vvsintW<'_, IenSpec> {
        R5vvsintW::new(self, 23)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn em23wakeup(&mut self) -> Em23wakeupW<'_, IenSpec> {
        Em23wakeupW::new(self, 24)
    }
    #[doc = "Bit 25 - VSCALEDONE Interrupt Enable"]
    #[inline(always)]
    pub fn vscaledone(&mut self) -> VscaledoneW<'_, IenSpec> {
        VscaledoneW::new(self, 25)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    pub fn temp(&mut self) -> TempW<'_, IenSpec> {
        TempW::new(self, 29)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    pub fn templow(&mut self) -> TemplowW<'_, IenSpec> {
        TemplowW::new(self, 30)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TemphighW<'_, IenSpec> {
        TemphighW::new(self, 31)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
